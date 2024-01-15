#include "controllers/api.hpp"
#include <jwt-cpp/jwt.h>

namespace Api {

void AuthController::logout(Request &req, Callback &&callback) {
  Json::Value json;
  json["message"] = "Logout";

  auto resp = drogon::HttpResponse::newHttpJsonResponse(json);
  callback(resp);
}

} // namespace Api
