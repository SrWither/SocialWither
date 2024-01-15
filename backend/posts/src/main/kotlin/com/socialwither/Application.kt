package com.socialwither

import com.socialwither.plugins.*
import io.ktor.server.application.*
import io.ktor.server.auth.*
import io.ktor.server.auth.jwt.*
import io.ktor.server.engine.*
import io.ktor.server.netty.*

/**
 * Main function to start the Ktor application.
 */
fun main() {
    // Start the Ktor application using the Netty engine
    embeddedServer(Netty, port = 4000, host = "0.0.0.0", module = Application::module)
        .start(wait = true)
}

/**
 * Ktor application module configuration.
 */
fun Application.module() {
    configureSerialization()
    configureHTTP()
    configureRouting()
}
