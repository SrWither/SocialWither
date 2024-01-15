#include "controllers/api.hpp"
#include <jwt-cpp/jwt.h>

namespace Api {

void AuthController::refreshToken(Request &req, Callback &&callback) {
  Json::Value json;
  json["message"] = "Refresh Token";

  auto resp = drogon::HttpResponse::newHttpJsonResponse(json);
  callback(resp);
}
} // namespace Api
