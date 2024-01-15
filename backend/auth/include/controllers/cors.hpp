#pragma once

#include "drogon/HttpFilter.h"
namespace Api {
class CorsFilter : public drogon::HttpFilter<CorsFilter, false> {
public:
  virtual void doFilter(const drogon::HttpRequestPtr &req,
                        drogon::FilterCallback &&fcb,
                        drogon::FilterChainCallback &&fccb) override;
};
} // namespace Api
