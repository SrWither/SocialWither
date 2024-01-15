#include "controllers/api.hpp"
#include <drogon/HttpClient.h>
#include <drogon/HttpTypes.h>

namespace Api {
void AuthController::registerUser(Request &req, Callback &&callback) {
  auto jsonReq = req->jsonObject();

  auto userApi = drogon::HttpClient::newHttpClient(std::getenv("USERAPI"));
  auto userApiReq = drogon::HttpRequest::newHttpRequest();
  userApiReq->setPath("/register");
  userApiReq->setContentTypeCode(drogon::CT_APPLICATION_JSON);
  userApiReq->setMethod(drogon::Post);
  userApiReq->setBody(jsonReq->toStyledString());

  userApi->sendRequest(
      userApiReq,
      [callback = std::move(callback)](
          drogon::ReqResult result, const drogon::HttpResponsePtr &response) {
        if (result == drogon::ReqResult::Ok) {
          if (response->statusCode() ==
                  drogon::HttpStatusCode::k401Unauthorized ||
              response->statusCode() ==
                  drogon::HttpStatusCode::k400BadRequest ||
              response->statusCode() ==
                  drogon::HttpStatusCode::k500InternalServerError) {
            callback(response);
          } else {
            std::string_view userApiResponse = response->body();
            Json::CharReaderBuilder readerBuilder;
            std::istringstream jsonStream(
                std::string(userApiResponse.data(), userApiResponse.length()));
            Json::Value apiUserJsonValue;
            Json::parseFromStream(readerBuilder, jsonStream, &apiUserJsonValue,
                                  nullptr);

            std::string token =
                generateJwtToken(apiUserJsonValue["id"].asString());

            Json::Value json;
            json["token"] = token;
            auto resp = drogon::HttpResponse::newHttpJsonResponse(json);
            resp->addHeader("Access-Control-Allow-Origin", "*");

            resp->addHeader("Access-Control-Allow-Methods",
                            "OPTIONS, POST, GET, PUT, PATCH");
            resp->addHeader("Access-Control-Allow-Headers", "*");

            resp->addHeader("Access-Control-Allow-Credentials", "true");

            resp->addHeader("Access-Control-Max-Age", "86400");
            callback(resp);
          }
        } else {
          callback(response);
        }
      });
}
} // namespace Api
