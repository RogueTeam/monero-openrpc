#pragma once
#include "test_defs.h"

namespace cryptonote
{
  class test_server
  {
  public:
    BEGIN_URI_MAP2()
      MAP_URI_AUTO_JON2("/get_thing", on_get_thing, COMMAND_RPC_GET_THING)
      MAP_URI_AUTO_BIN2("/get_thing.bin", on_get_thing_bin, COMMAND_RPC_GET_THING)
      BEGIN_JSON_RPC_MAP("/json_rpc")
        MAP_JON_RPC_WE("get_thing", on_get_thing_json, COMMAND_RPC_GET_THING)
        MAP_JON_RPC_WE("getthing", on_get_thing_json, COMMAND_RPC_GET_THING)
        MAP_JON_RPC_WE_IF("get_other", on_get_other, COMMAND_RPC_GET_OTHER, !m_restricted)
        MAP_JON_RPC_WE("get_count", on_get_count, COMMAND_RPC_GET_COUNT)
        MAP_JON_RPC_WE("hash_of", on_hash_of, COMMAND_RPC_HASH_OF)
      END_JSON_RPC_MAP()
    END_URI_MAP2()

    bool on_get_count(const COMMAND_RPC_GET_COUNT::request& req, COMMAND_RPC_GET_COUNT::response& res, epee::json_rpc::error& error_resp, const connection_context *ctx = NULL);
    bool on_hash_of(const COMMAND_RPC_HASH_OF::request& req, COMMAND_RPC_HASH_OF::response& res, epee::json_rpc::error& error_resp, const connection_context *ctx = NULL);
  };
}
