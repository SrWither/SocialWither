package com.socialwither.plugins

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.plugins.cors.routing.*
import io.ktor.server.plugins.openapi.*
import io.ktor.server.plugins.swagger.*
import io.ktor.server.routing.*

/**
 * Configures various HTTP-related features in the Ktor application.
 */
fun Application.configureHTTP() {
    // Configure Swagger UI at the specified path ("/openapi")
    routing {
        swaggerUI(path = "openapi")
    }

    // Configure OpenAPI at the specified path ("/openapi")
    routing {
        openAPI(path = "openapi")
    }

    // Install Cross-Origin Resource Sharing (CORS) plugin
    install(CORS) {
        // Allow specific HTTP methods
        allowMethod(HttpMethod.Get)
        allowMethod(HttpMethod.Post)
        allowMethod(HttpMethod.Put)
        allowMethod(HttpMethod.Patch)
        allowMethod(HttpMethod.Delete)
        allowMethod(HttpMethod.Options)

        // Allow specific HTTP headers
        allowHeader(HttpHeaders.Authorization)
        allowHeader(HttpHeaders.ContentType)

        // Allow requests from any host (not recommended for production)
        anyHost()
    }
}
