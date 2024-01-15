package com.socialwither.plugins

import com.socialwither.databases.Mongo.*
import com.socialwither.models.Post.*
import com.auth0.jwt.JWT
import com.auth0.jwt.interfaces.JWTVerifier
import com.auth0.jwt.algorithms.Algorithm
import io.ktor.http.HttpStatusCode
import io.ktor.server.auth.*
import io.ktor.server.auth.jwt.*
import io.ktor.server.application.*
import io.ktor.server.request.receive
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.auth.*
import kotlinx.serialization.*
import kotlinx.serialization.json.*
import kotlinx.coroutines.withContext
import kotlinx.coroutines.Dispatchers
import java.util.Date

/**
 * Configures the routing for the Ktor application.
 */

@Serializable
data class ErrorMessage(
    val error: String,
)

@Serializable
data class SimpleMessage(
    val message: String,
)

data class JwtPayload(val userId: String)

object JwtConfig {
    private const val secret = "thisnotwebassembly"
    private const val validityInMs = 24 * 60 * 60 * 1000

    val verifier: JWTVerifier = JWT
        .require(Algorithm.HMAC256(secret))
        .build()

    fun makeJwt(userId: String): String = JWT.create()
        .withClaim("user_id", userId)
        .withExpiresAt(Date(System.currentTimeMillis() + validityInMs))
        .sign(Algorithm.HMAC256(secret))
}

@Serializable
data class JwtPrincipal(val payload: JwtPayload) : Principal

suspend fun validateJwt(call: ApplicationCall) {
    val token = call.request.headers["Authorization"]?.removePrefix("Bearer ")
    if (token == null) {
        call.respond(HttpStatusCode.Unauthorized, ErrorMessage("Token not present"))
        throw IllegalStateException("No credentials: Token not present")
    }
    try {
        withContext(Dispatchers.IO) {
            JwtConfig.verifier.verify(token)
        }
    } catch (e: Exception) {
        call.respond(HttpStatusCode.Forbidden, ErrorMessage("Invalid token"))
        throw IllegalStateException("Invalid credentials: Invalid token", e)
    }
}

fun Application.configureRouting() {
    // Create an instance of the PostService for handling Post-related operations
    val service = PostService()

    install(Authentication) {
        jwt("auth-jwt") {
            verifier(JwtConfig.verifier)
            validate { credential ->
                if (credential.payload.getClaim("user_id").asString() != "") {
                    JWTPrincipal(credential.payload)
                } else {
                    null
                }
            }
            challenge { _, _ ->
                validateJwt(call)
            }
        }
    }

    routing {
        // Hello World endpoint
        get("/") { call.respondText("Hello World!") }

        // Routes related to posts

            route("/post") {
                // Create Post endpoint
                authenticate("auth-jwt") {
                    post {
                        val principal = call.principal<JWTPrincipal>()
                        val userId = principal!!.payload.getClaim("user_id").asString()
                        println(userId)
                        val request = call.receive<PostDto>().setUserId(userId.orEmpty())
                        val post = request.toPost()

                        // Attempt to create the post
                        service.create(post)?.let { data ->
                            call.respond(HttpStatusCode.Created, data)
                        } ?: call.respond(HttpStatusCode.BadRequest, ErrorMessage("Error creating post"))
                    }
                }

            // Read Post endpoint
            get("/{id}") {
                val id = call.parameters["id"].toString()

                // Validate the length of the ID
                if (id.length < 24) {
                    call.respond(HttpStatusCode.BadRequest, ErrorMessage("Invalid ID length"))
                    return@get
                }

                // Attempt to retrieve and respond with the post
                service.get(id)?.let { data ->
                    call.respond(HttpStatusCode.OK, data)
                } ?: call.respond(HttpStatusCode.NotFound, ErrorMessage("Post not found"))
            }

            // Read All endpoint
            get {
                // Retrieve all posts and respond with the list
                val posts = service.getAll()
                call.respond(HttpStatusCode.OK, posts)
            }

            // Update Post endpoint
            authenticate("auth-jwt") {
                put("/{id}") {
                    val principal = call.principal<JWTPrincipal>()
                    val userIdFromToken = principal!!.payload.getClaim("user_id").asString()
                    val id = call.parameters["id"].toString()

                    // Validate the length of the ID
                    if (id.length < 24) {
                        call.respond(HttpStatusCode.BadRequest, ErrorMessage("Invalid ID length"))
                        return@put
                    }

                    val request = call.receive<PostDto>().setUserId(userIdFromToken.orEmpty())
                    val post = request.toPost()

                    // Retrieve the existing post
                    val existingPost = service.get(id)

                    // Check if the post exists
                    if (existingPost != null) {
                        // Check if the user_id from the token matches the user_id of the post
                        if (userIdFromToken == existingPost.user_id) {
                            // Attempt to update the post
                            service.update(id, post)?.let { data ->
                                call.respond(HttpStatusCode.OK, data)
                            } ?: call.respond(HttpStatusCode.NotFound, ErrorMessage("Post not found"))
                        } else {
                            call.respond(HttpStatusCode.Forbidden, ErrorMessage("You cannot edit this post"))
                        }
                    } else {
                        call.respond(HttpStatusCode.NotFound, ErrorMessage("Post not found"))
                    }
                }
            }

            // Delete Post endpoint
            authenticate("auth-jwt") {
                delete("/{id}") {
                    val principal = call.principal<JWTPrincipal>()
                    val userIdFromToken = principal!!.payload.getClaim("user_id").asString()
                    val id = call.parameters["id"].toString()

                    // Validate the length of the ID
                    if (id.length < 24) {
                        call.respond(HttpStatusCode.BadRequest, ErrorMessage("Invalid ID length"))
                        return@delete
                    }

                    // Retrieve the existing post
                    val existingPost = service.get(id)

                    // Check if the post exists
                    if (existingPost != null) {
                        // Check if the user_id from the token matches the user_id of the post
                        if (userIdFromToken == existingPost.user_id) {
                            // Attempt to delete the post
                            val result = service.delete(id)
                            if (result) {
                                call.respond(HttpStatusCode.OK, SimpleMessage("Post deleted"))
                            } else {
                                call.respond(HttpStatusCode.NotFound, ErrorMessage("Post not found"))
                            }
                        } else {
                            call.respond(HttpStatusCode.Forbidden, ErrorMessage("You cannot delete this post"))
                        }
                    } else {
                        call.respond(HttpStatusCode.NotFound, ErrorMessage("Post not found"))
                    }
                }
            }

        }
    }
}
