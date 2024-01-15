#include "controllers/cors.hpp"

namespace Api {
  void CorsFilter::doFilter(const drogon::HttpRequestPtr &req,
                           drogon::FilterCallback &&fcb,
                           drogon::FilterChainCallback &&fccb) {
    auto resp = drogon::HttpResponse::newHttpResponse();
    auto &origin = req->getHeader("Origin");
    
    resp->addHeader("Access-Control-Allow-Origin", "*");
    
    resp->addHeader("Access-Control-Allow-Methods", "OPTIONS, POST, GET, PUT, PATCH");
    resp->addHeader("Access-Control-Allow-Headers", "*");
    
    resp->addHeader("Access-Control-Allow-Credentials", "true");
    
    resp->addHeader("Access-Control-Max-Age", "86400");

    if (req->method() == drogon::Options) {
        fcb(resp);
    } else {
        fccb();
    }
  }
}
