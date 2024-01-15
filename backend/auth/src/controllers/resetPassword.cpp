#include "controllers/api.hpp"
#include <jwt-cpp/jwt.h>

namespace Api {

void AuthController::resetPassword(Request &req, Callback &&callback) {
  Json::Value json;
  json["message"] = "Reset Password";

  auto resp = drogon::HttpResponse::newHttpJsonResponse(json);
  callback(resp);
}
} // namespace Api
