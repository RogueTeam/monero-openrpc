// A small stand-in for core_rpc_server_commands_defs.h, written in the same style,
// with one example of each construct the generator has to handle.
#pragma once

#define TEST_RPC_VERSION_MAJOR 3
#define TEST_RPC_VERSION_MINOR 7
#define SOME_MACRO(x) \
  struct not_a_struct_##x {};

namespace cryptonote
{
  struct rpc_response_base
  {
    std::string status;

    BEGIN_KV_SERIALIZE_MAP()
      KV_SERIALIZE(status)
    END_KV_SERIALIZE_MAP()
  };

  struct entry
  {
    uint64_t a, b = 0;              // several names in one declaration
    std::string hash{};

    BEGIN_KV_SERIALIZE_MAP()
      KV_SERIALIZE(a)
      KV_SERIALIZE_N(b, "bee")
      KV_SERIALIZE(hash)
    END_KV_SERIALIZE_MAP()
  };

  struct raw_holder
  {
    std::vector<uint64_t> list;

    BEGIN_KV_SERIALIZE_MAP()
      std::vector<uint32_t> narrowed;
      epee::serialization::selector<is_store>::serialize(narrowed, stg, hparent_section, "list");
      if (this_ref.list.empty()) KV_SERIALIZE(list)
    END_KV_SERIALIZE_MAP()
  };

  struct COMMAND_RPC_GET_THING
  {
    struct request_t
    {
      uint32_t count; // how many
      bool verbose;
      std::string tag; //* a // comment that looks like a block comment */
      std::string name;
      std::vector<uint64_t> heights;

      BEGIN_KV_SERIALIZE_MAP()
        KV_SERIALIZE_OPT(count, (uint32_t)10)
        KV_SERIALIZE_OPT(verbose, false)
        KV_SERIALIZE_OPT(tag, std::string())
        KV_SERIALIZE(name)
        KV_SERIALIZE_OPT(heights, std::vector<uint64_t>())
      END_KV_SERIALIZE_MAP()
    };
    typedef epee::misc_utils::struct_init<request_t> request;

    struct response_t: public rpc_response_base
    {
      int8_t small;
      uint64_t big;
      double ratio;
      std::vector<entry> entries;
      crypto::hash raw;
      boost::optional<uint64_t> maybe;
      std::string mode;
      uint64_t height;
      /* a block comment
         struct ignored { int x; };
      */

      BEGIN_KV_SERIALIZE_MAP()
        KV_SERIALIZE_PARENT(rpc_response_base)
        KV_SERIALIZE(small)
        KV_SERIALIZE(big)
        KV_SERIALIZE(ratio)
        KV_SERIALIZE(entries)
        KV_SERIALIZE_VAL_POD_AS_BLOB(raw)
        KV_SERIALIZE(maybe)
        if (this_ref.mode == "height")
        {
          KV_SERIALIZE_N(height, "value")
        }
        else
          KV_SERIALIZE_N(mode, "value")
      END_KV_SERIALIZE_MAP()
    };
    typedef epee::misc_utils::struct_init<response_t> response;
  };

  struct COMMAND_RPC_GET_OTHER
  {
    typedef COMMAND_RPC_GET_THING::request request;

    struct response_t: public COMMAND_RPC_GET_THING::response_t
    {
    };
    typedef epee::misc_utils::struct_init<response_t> response;
  };

  struct COMMAND_RPC_GET_COUNT
  {
    typedef std::list<std::string> request;

    struct response_t: public rpc_response_base
    {
      uint64_t count;
      raw_holder holder;

      BEGIN_KV_SERIALIZE_MAP()
        KV_SERIALIZE_PARENT(rpc_response_base)
        KV_SERIALIZE(count)
        KV_SERIALIZE(holder)
      END_KV_SERIALIZE_MAP()
    };
    typedef epee::misc_utils::struct_init<response_t> response;
  };

  struct COMMAND_RPC_HASH_OF
  {
    typedef std::vector<uint64_t> request;
    typedef std::string response;
  };
}
