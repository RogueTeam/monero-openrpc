#include "test_server.h"

namespace cryptonote
{
  bool test_server::on_get_count(const COMMAND_RPC_GET_COUNT::request& req, COMMAND_RPC_GET_COUNT::response& res, epee::json_rpc::error& error_resp, const connection_context *ctx)
  {
    res.count = 42;
    res.status = CORE_RPC_STATUS_OK;
    return true;
  }

  bool test_server::on_hash_of(const COMMAND_RPC_HASH_OF::request& req, COMMAND_RPC_HASH_OF::response& res, epee::json_rpc::error& error_resp, const connection_context *ctx)
  {
    if (req.size() != 1)
    {
      error_resp.code = TEST_RPC_ERROR_CODE_WRONG_PARAM;
      error_resp.message = "Wrong parameters, expected height";
      return false;
    }
    res = "00";
    return true;
  }
}
