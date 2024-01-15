#include "controllers/cors.hpp"
#include "controllers/api.hpp"
#include "dotenv.hpp"
#include <drogon/drogon.h>

int main() {

  dotenv::init();

  drogon::app().addListener("0.0.0.0", 8080);

  drogon::app().registerFilter(std::make_shared<Api::CorsFilter>());

  drogon::app().registerController(std::make_shared<Api::AuthController>());

  drogon::app().run();
  return 0;
}
