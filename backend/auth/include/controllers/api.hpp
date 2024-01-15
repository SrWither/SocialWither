#pragma once

#include <drogon/HttpController.h>
namespace Api {
using Request = const drogon::HttpRequestPtr &;
using Response = const drogon::HttpResponsePtr &;
using Callback = std::function<void(Response)>;

class AuthController : public drogon::HttpController<AuthController, false> {
public:
  METHOD_LIST_BEGIN
  ADD_METHOD_TO(AuthController::registerUser, "/auth/register", drogon::Post,
             drogon::Options, "Api::CorsFilter");
  ADD_METHOD_TO(AuthController::login, "/auth/login", drogon::Post,
             drogon::Options, "Api::CorsFilter");
  ADD_METHOD_TO(AuthController::getUser, "/auth/user", drogon::Get,
             drogon::Options, "Api::CorsFilter");
  ADD_METHOD_TO(AuthController::getProfile, "/auth/profile", drogon::Get,
             drogon::Options, "Api::CorsFilter");
  ADD_METHOD_TO(AuthController::refreshToken, "/auth/refresh", drogon::Post,
             drogon::Options, "Api::CorsFilter");
  ADD_METHOD_TO(AuthController::logout, "/auth/logout", drogon::Post,
             drogon::Options, "Api::CorsFilter");
  ADD_METHOD_TO(AuthController::verifyToken, "/auth/verify", drogon::Get,
             drogon::Options, "Api::CorsFilter");
  ADD_METHOD_TO(AuthController::forgotPassword, "/auth/forgot-password",
             drogon::Post, drogon::Options, "Api::CorsFilter");
  ADD_METHOD_TO(AuthController::resetPassword, "/auth/reset-password",
             drogon::Post, drogon::Options, "Api::CorsFilter");
  METHOD_LIST_END

  void registerUser(Request &req, Callback &&callback);

  void login(Request &req, Callback &&callback);

  void getUser(Request &req, Callback &&callback);

  void getProfile(Request &req, Callback &&callback);

  void refreshToken(Request &req, Callback &&callback);

  void logout(Request &req, Callback &&callback);

  void verifyToken(Request &req, Callback &&callback);

  void forgotPassword(Request &req, Callback &&callback);

  void resetPassword(Request &req, Callback &&callback);
};

std::string generateJwtToken(std::string userId);
std::string getUserIdByToken(std::string token);

} // namespace Api
