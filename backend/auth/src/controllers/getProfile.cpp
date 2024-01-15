#include "controllers/api.hpp"
#include <drogon/HttpClient.h>
#include <fmt/format.h>

namespace Api {
void AuthController::getProfile(Request &req, Callback &&callback) {
  std::string token = req->getHeader("Authorization");
  std::string userId = getUserIdByToken(token.substr(7));

  auto userApi = drogon::HttpClient::newHttpClient(std::getenv("USERAPI"));
  auto userApiReq = drogon::HttpRequest::newHttpRequest();
  userApiReq->setPath(fmt::format("/profiles/{}", userId));
  userApiReq->setContentTypeCode(drogon::CT_APPLICATION_JSON);
  userApiReq->setMethod(drogon::Get);

  userApi->sendRequest(
      userApiReq,
      [callback = std::move(callback)](
          drogon::ReqResult result, const drogon::HttpResponsePtr &resp) {
        if (result == drogon::ReqResult::Ok) {
          resp->addHeader("Access-Control-Allow-Origin", "*");

          resp->addHeader("Access-Control-Allow-Methods",
                          "OPTIONS, POST, GET, PUT, PATCH");
          resp->addHeader("Access-Control-Allow-Headers", "*");

          resp->addHeader("Access-Control-Allow-Credentials", "true");

          resp->addHeader("Access-Control-Max-Age", "86400");
          callback(resp);
        } else {
          callback(resp);
        }
      });
}
} // namespace Api
