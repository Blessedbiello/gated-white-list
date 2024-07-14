/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/whitelist_token_sale.json`.
 */
export type WhitelistTokenSale = {
  "address": "3sUi78FsNDGBK12dBj4MZNJRpQAaokW6gTs9NrHjfH3P",
  "metadata": {
    "name": "whitelistTokenSale",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "addToWhitelist",
      "docs": [
        "* Add the account `account_to_add` with the specified bump to the `whitelist` account."
      ],
      "discriminator": [
        157,
        211,
        52,
        54,
        144,
        81,
        5,
        55
      ],
      "accounts": [
        {
          "name": "entry",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "arg",
                "path": "accountToAdd"
              },
              {
                "kind": "account",
                "path": "whitelist"
              }
            ]
          }
        },
        {
          "name": "whitelist"
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true,
          "relations": [
            "whitelist"
          ]
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "accountToAdd",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "buyTokens",
      "discriminator": [
        189,
        21,
        230,
        133,
        247,
        2,
        110,
        42
      ],
      "accounts": [
        {
          "name": "buyer",
          "writable": true,
          "signer": true
        },
        {
          "name": "sale",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  97,
                  108,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "sale.authority",
                "account": "sale"
              },
              {
                "kind": "account",
                "path": "sale.whitelist",
                "account": "sale"
              }
            ]
          }
        },
        {
          "name": "whitelistEntry",
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "buyer"
              },
              {
                "kind": "account",
                "path": "sale.whitelist",
                "account": "sale"
              }
            ]
          }
        },
        {
          "name": "user",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "buyer"
              },
              {
                "kind": "account",
                "path": "sale"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "checkWhitelist",
      "docs": [
        "* Check if the given whitelist exists (has been initalized)."
      ],
      "discriminator": [
        202,
        135,
        140,
        249,
        143,
        1,
        201,
        143
      ],
      "accounts": [
        {
          "name": "whitelist",
          "pda": {
            "seeds": [
              {
                "kind": "arg",
                "path": "owner"
              },
              {
                "kind": "arg",
                "path": "name"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "owner",
          "type": "pubkey"
        },
        {
          "name": "name",
          "type": "string"
        }
      ]
    },
    {
      "name": "checkWhitelisted",
      "docs": [
        "* Check if the account `account_to_check` is whitelisted in the given `whitelist` account."
      ],
      "discriminator": [
        55,
        32,
        174,
        31,
        5,
        246,
        187,
        13
      ],
      "accounts": [
        {
          "name": "entry",
          "pda": {
            "seeds": [
              {
                "kind": "arg",
                "path": "accountToCheck"
              },
              {
                "kind": "account",
                "path": "whitelist"
              }
            ]
          }
        },
        {
          "name": "whitelist"
        }
      ],
      "args": [
        {
          "name": "accountToCheck",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "deleteWhitelist",
      "docs": [
        "* Delete a whitelist create with `init_whitelist` with the given name and bump,\n     * associated with the authority of this transaction. The remaining lamports are sent to the\n     * authority of this transaction."
      ],
      "discriminator": [
        229,
        26,
        200,
        168,
        119,
        177,
        128,
        19
      ],
      "accounts": [
        {
          "name": "whitelist",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "authority"
              },
              {
                "kind": "arg",
                "path": "name"
              }
            ]
          }
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        }
      ]
    },
    {
      "name": "initSale",
      "discriminator": [
        41,
        197,
        251,
        217,
        167,
        153,
        95,
        49
      ],
      "accounts": [
        {
          "name": "sale",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  97,
                  108,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              },
              {
                "kind": "account",
                "path": "whitelist"
              }
            ]
          }
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true,
          "relations": [
            "whitelist"
          ]
        },
        {
          "name": "whitelist"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "tokenPrice",
          "type": "u64"
        },
        {
          "name": "maxPerWallet",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initWhitelist",
      "docs": [
        "* Initialize a whitelist with the given name, associated with the authority of this transaction."
      ],
      "discriminator": [
        103,
        193,
        162,
        241,
        234,
        97,
        110,
        71
      ],
      "accounts": [
        {
          "name": "whitelist",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "authority"
              },
              {
                "kind": "arg",
                "path": "name"
              }
            ]
          }
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        }
      ]
    },
    {
      "name": "removeFromWhitelist",
      "docs": [
        "* Remove the account `account_to_delete` with the specified bump from the `whitelist` account.\n     * Remaining lamports are sent to the authority of this transaction."
      ],
      "discriminator": [
        7,
        144,
        216,
        239,
        243,
        236,
        193,
        235
      ],
      "accounts": [
        {
          "name": "entry",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "arg",
                "path": "accountToDelete"
              },
              {
                "kind": "account",
                "path": "whitelist"
              }
            ]
          }
        },
        {
          "name": "whitelist"
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true,
          "relations": [
            "whitelist"
          ]
        }
      ],
      "args": [
        {
          "name": "accountToDelete",
          "type": "pubkey"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "sale",
      "discriminator": [
        202,
        64,
        232,
        171,
        178,
        172,
        34,
        183
      ]
    },
    {
      "name": "user",
      "discriminator": [
        159,
        117,
        95,
        227,
        239,
        151,
        58,
        236
      ]
    },
    {
      "name": "whitelistEntry",
      "discriminator": [
        51,
        70,
        173,
        81,
        219,
        192,
        234,
        62
      ]
    },
    {
      "name": "whitelistTokenSale",
      "discriminator": [
        70,
        158,
        0,
        238,
        229,
        63,
        136,
        60
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "maxPurchaseLimitReached",
      "msg": "Max purchase limit reached"
    },
    {
      "code": 6001,
      "name": "insufficientFunds",
      "msg": "Insufficient funds"
    }
  ],
  "types": [
    {
      "name": "sale",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "tokenPrice",
            "type": "u64"
          },
          {
            "name": "maxPerWallet",
            "type": "u64"
          },
          {
            "name": "whitelist",
            "type": "pubkey"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "user",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "purchasedAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "whitelistEntry",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "parent",
            "type": "pubkey"
          },
          {
            "name": "whitelisted",
            "type": "pubkey"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "whitelistTokenSale",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "name",
            "type": "string"
          }
        ]
      }
    }
  ]
};
