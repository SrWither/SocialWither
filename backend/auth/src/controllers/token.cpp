#include "controllers/api.hpp"
#include <jwt-cpp/jwt.h>

namespace Api {
std::string generateJwtToken(std::string userId) {
  using namespace jwt;

  auto payload = jwt::create()
                     .set_type("JWT")
                     .set_payload_claim("user_id", jwt::claim(userId))
                     .set_expires_at(std::chrono::system_clock::now() +
                                     std::chrono::hours{24})
                     .sign(jwt::algorithm::hs256{std::getenv("SECRETKEY")});

  return payload;
}

std::string getUserIdByToken(std::string token) {
  auto decoded = jwt::decode(token);
  auto userId = decoded.get_payload_claim("user_id");
  return userId.as_string();
}

} // namespace Api
