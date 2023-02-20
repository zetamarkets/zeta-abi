export type AbiWrapper = {
  "version": "0.1.0",
  "name": "abi_wrapper",
  "instructions": [
    {
      "name": "initializeMarginAccount",
      "accounts": [
        {
          "name": "marginAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zetaGroup",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initializeOpenOrders",
      "accounts": [
        {
          "name": "state",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zetaGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "dexProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "openOrders",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marginAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "market",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "serumAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "openOrdersMap",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "deposit",
      "accounts": [
        {
          "name": "zetaGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "socializedLossAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "state",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "greeks",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
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
      "name": "withdraw",
      "accounts": [
        {
          "name": "zetaGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "socializedLossAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "state",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "greeks",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "oracleBackupFeed",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracleBackupProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
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
      "name": "placeOrderV4",
      "accounts": [
        {
          "name": "state",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zetaGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "dexProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "serumAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "greeks",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "openOrders",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketAccounts",
          "accounts": [
            {
              "name": "market",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "requestQueue",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "eventQueue",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "bids",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "asks",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "orderPayerTokenAccount",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "coinVault",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "pcVault",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "coinWallet",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "pcWallet",
              "isMut": true,
              "isSigner": false
            }
          ]
        },
        {
          "name": "oracle",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracleBackupFeed",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracleBackupProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketNode",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marketMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "price",
          "type": "u64"
        },
        {
          "name": "size",
          "type": "u64"
        },
        {
          "name": "side",
          "type": {
            "defined": "Side"
          }
        },
        {
          "name": "orderType",
          "type": {
            "defined": "OrderType"
          }
        },
        {
          "name": "clientOrderId",
          "type": {
            "option": "u64"
          }
        },
        {
          "name": "tag",
          "type": {
            "option": "string"
          }
        },
        {
          "name": "tifOffset",
          "type": {
            "option": "u16"
          }
        }
      ]
    },
    {
      "name": "placePerpOrderV2",
      "accounts": [
        {
          "name": "state",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zetaGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "dexProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "serumAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "greeks",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "openOrders",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketAccounts",
          "accounts": [
            {
              "name": "market",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "requestQueue",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "eventQueue",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "bids",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "asks",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "orderPayerTokenAccount",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "coinVault",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "pcVault",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "coinWallet",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "pcWallet",
              "isMut": true,
              "isSigner": false
            }
          ]
        },
        {
          "name": "oracle",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracleBackupFeed",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracleBackupProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "perpSyncQueue",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "price",
          "type": "u64"
        },
        {
          "name": "size",
          "type": "u64"
        },
        {
          "name": "side",
          "type": {
            "defined": "Side"
          }
        },
        {
          "name": "orderType",
          "type": {
            "defined": "OrderType"
          }
        },
        {
          "name": "clientOrderId",
          "type": {
            "option": "u64"
          }
        },
        {
          "name": "tag",
          "type": {
            "option": "string"
          }
        },
        {
          "name": "tifOffset",
          "type": {
            "option": "u16"
          }
        }
      ]
    },
    {
      "name": "cancelOrder",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "cancelAccounts",
          "accounts": [
            {
              "name": "zetaGroup",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "state",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "marginAccount",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "dexProgram",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "serumAuthority",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "openOrders",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "market",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "bids",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "asks",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "eventQueue",
              "isMut": true,
              "isSigner": false
            }
          ]
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "side",
          "type": {
            "defined": "Side"
          }
        },
        {
          "name": "orderId",
          "type": "u128"
        }
      ]
    },
    {
      "name": "cancelOrderNoError",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "cancelAccounts",
          "accounts": [
            {
              "name": "zetaGroup",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "state",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "marginAccount",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "dexProgram",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "serumAuthority",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "openOrders",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "market",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "bids",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "asks",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "eventQueue",
              "isMut": true,
              "isSigner": false
            }
          ]
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "side",
          "type": {
            "defined": "Side"
          }
        },
        {
          "name": "orderId",
          "type": "u128"
        }
      ]
    },
    {
      "name": "cancelAllMarketOrders",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "cancelAccounts",
          "accounts": [
            {
              "name": "zetaGroup",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "state",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "marginAccount",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "dexProgram",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "serumAuthority",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "openOrders",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "market",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "bids",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "asks",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "eventQueue",
              "isMut": true,
              "isSigner": false
            }
          ]
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "cancelOrderByClientOrderId",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "cancelAccounts",
          "accounts": [
            {
              "name": "zetaGroup",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "state",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "marginAccount",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "dexProgram",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "serumAuthority",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "openOrders",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "market",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "bids",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "asks",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "eventQueue",
              "isMut": true,
              "isSigner": false
            }
          ]
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "clientOrderId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "cancelOrderByClientOrderIdNoError",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "cancelAccounts",
          "accounts": [
            {
              "name": "zetaGroup",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "state",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "marginAccount",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "dexProgram",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "serumAuthority",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "openOrders",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "market",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "bids",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "asks",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "eventQueue",
              "isMut": true,
              "isSigner": false
            }
          ]
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "clientOrderId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "forceCancelOrders",
      "accounts": [
        {
          "name": "greeks",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracle",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracleBackupFeed",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracleBackupProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "cancelAccounts",
          "accounts": [
            {
              "name": "zetaGroup",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "state",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "marginAccount",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "dexProgram",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "serumAuthority",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "openOrders",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "market",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "bids",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "asks",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "eventQueue",
              "isMut": true,
              "isSigner": false
            }
          ]
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "closeMargin",
      "accounts": [
        {
          "name": "marginAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "zetaGroup",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "closeOpenOrders",
      "accounts": [
        {
          "name": "state",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zetaGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "dexProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "openOrders",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marginAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "market",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "serumAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "openOrdersMap",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "mapNonce",
          "type": "u8"
        }
      ]
    },
    {
      "name": "liquidate",
      "accounts": [
        {
          "name": "state",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "liquidator",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "liquidatorMarginAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "greeks",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracle",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracleBackupFeed",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracleBackupProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "market",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zetaGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "liquidatedMarginAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    }
  ],
  "types": [
    {
      "name": "Side",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Uninitialized"
          },
          {
            "name": "Bid"
          },
          {
            "name": "Ask"
          }
        ]
      }
    },
    {
      "name": "OrderType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Limit"
          },
          {
            "name": "PostOnly"
          },
          {
            "name": "FillOrKill"
          },
          {
            "name": "ImmediateOrCancel"
          },
          {
            "name": "PostOnlySlide"
          }
        ]
      }
    }
  ]
};

export const IDL: AbiWrapper = {
  "version": "0.1.0",
  "name": "abi_wrapper",
  "instructions": [
    {
      "name": "initializeMarginAccount",
      "accounts": [
        {
          "name": "marginAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zetaGroup",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initializeOpenOrders",
      "accounts": [
        {
          "name": "state",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zetaGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "dexProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "openOrders",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marginAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "market",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "serumAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "openOrdersMap",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "deposit",
      "accounts": [
        {
          "name": "zetaGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "socializedLossAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "state",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "greeks",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
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
      "name": "withdraw",
      "accounts": [
        {
          "name": "zetaGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "socializedLossAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "state",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "greeks",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "oracleBackupFeed",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracleBackupProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
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
      "name": "placeOrderV4",
      "accounts": [
        {
          "name": "state",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zetaGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "dexProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "serumAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "greeks",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "openOrders",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketAccounts",
          "accounts": [
            {
              "name": "market",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "requestQueue",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "eventQueue",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "bids",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "asks",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "orderPayerTokenAccount",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "coinVault",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "pcVault",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "coinWallet",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "pcWallet",
              "isMut": true,
              "isSigner": false
            }
          ]
        },
        {
          "name": "oracle",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracleBackupFeed",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracleBackupProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketNode",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marketMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "price",
          "type": "u64"
        },
        {
          "name": "size",
          "type": "u64"
        },
        {
          "name": "side",
          "type": {
            "defined": "Side"
          }
        },
        {
          "name": "orderType",
          "type": {
            "defined": "OrderType"
          }
        },
        {
          "name": "clientOrderId",
          "type": {
            "option": "u64"
          }
        },
        {
          "name": "tag",
          "type": {
            "option": "string"
          }
        },
        {
          "name": "tifOffset",
          "type": {
            "option": "u16"
          }
        }
      ]
    },
    {
      "name": "placePerpOrderV2",
      "accounts": [
        {
          "name": "state",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zetaGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "dexProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "serumAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "greeks",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "openOrders",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketAccounts",
          "accounts": [
            {
              "name": "market",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "requestQueue",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "eventQueue",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "bids",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "asks",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "orderPayerTokenAccount",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "coinVault",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "pcVault",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "coinWallet",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "pcWallet",
              "isMut": true,
              "isSigner": false
            }
          ]
        },
        {
          "name": "oracle",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracleBackupFeed",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracleBackupProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "perpSyncQueue",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "price",
          "type": "u64"
        },
        {
          "name": "size",
          "type": "u64"
        },
        {
          "name": "side",
          "type": {
            "defined": "Side"
          }
        },
        {
          "name": "orderType",
          "type": {
            "defined": "OrderType"
          }
        },
        {
          "name": "clientOrderId",
          "type": {
            "option": "u64"
          }
        },
        {
          "name": "tag",
          "type": {
            "option": "string"
          }
        },
        {
          "name": "tifOffset",
          "type": {
            "option": "u16"
          }
        }
      ]
    },
    {
      "name": "cancelOrder",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "cancelAccounts",
          "accounts": [
            {
              "name": "zetaGroup",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "state",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "marginAccount",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "dexProgram",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "serumAuthority",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "openOrders",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "market",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "bids",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "asks",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "eventQueue",
              "isMut": true,
              "isSigner": false
            }
          ]
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "side",
          "type": {
            "defined": "Side"
          }
        },
        {
          "name": "orderId",
          "type": "u128"
        }
      ]
    },
    {
      "name": "cancelOrderNoError",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "cancelAccounts",
          "accounts": [
            {
              "name": "zetaGroup",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "state",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "marginAccount",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "dexProgram",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "serumAuthority",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "openOrders",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "market",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "bids",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "asks",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "eventQueue",
              "isMut": true,
              "isSigner": false
            }
          ]
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "side",
          "type": {
            "defined": "Side"
          }
        },
        {
          "name": "orderId",
          "type": "u128"
        }
      ]
    },
    {
      "name": "cancelAllMarketOrders",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "cancelAccounts",
          "accounts": [
            {
              "name": "zetaGroup",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "state",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "marginAccount",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "dexProgram",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "serumAuthority",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "openOrders",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "market",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "bids",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "asks",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "eventQueue",
              "isMut": true,
              "isSigner": false
            }
          ]
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "cancelOrderByClientOrderId",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "cancelAccounts",
          "accounts": [
            {
              "name": "zetaGroup",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "state",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "marginAccount",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "dexProgram",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "serumAuthority",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "openOrders",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "market",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "bids",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "asks",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "eventQueue",
              "isMut": true,
              "isSigner": false
            }
          ]
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "clientOrderId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "cancelOrderByClientOrderIdNoError",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "cancelAccounts",
          "accounts": [
            {
              "name": "zetaGroup",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "state",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "marginAccount",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "dexProgram",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "serumAuthority",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "openOrders",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "market",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "bids",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "asks",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "eventQueue",
              "isMut": true,
              "isSigner": false
            }
          ]
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "clientOrderId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "forceCancelOrders",
      "accounts": [
        {
          "name": "greeks",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracle",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracleBackupFeed",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracleBackupProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "cancelAccounts",
          "accounts": [
            {
              "name": "zetaGroup",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "state",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "marginAccount",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "dexProgram",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "serumAuthority",
              "isMut": false,
              "isSigner": false
            },
            {
              "name": "openOrders",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "market",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "bids",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "asks",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "eventQueue",
              "isMut": true,
              "isSigner": false
            }
          ]
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "closeMargin",
      "accounts": [
        {
          "name": "marginAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "zetaGroup",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "closeOpenOrders",
      "accounts": [
        {
          "name": "state",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zetaGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "dexProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "openOrders",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marginAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "market",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "serumAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "openOrdersMap",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "mapNonce",
          "type": "u8"
        }
      ]
    },
    {
      "name": "liquidate",
      "accounts": [
        {
          "name": "state",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "liquidator",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "liquidatorMarginAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "greeks",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracle",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracleBackupFeed",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracleBackupProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "market",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zetaGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "liquidatedMarginAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zetaProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    }
  ],
  "types": [
    {
      "name": "Side",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Uninitialized"
          },
          {
            "name": "Bid"
          },
          {
            "name": "Ask"
          }
        ]
      }
    },
    {
      "name": "OrderType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Limit"
          },
          {
            "name": "PostOnly"
          },
          {
            "name": "FillOrKill"
          },
          {
            "name": "ImmediateOrCancel"
          },
          {
            "name": "PostOnlySlide"
          }
        ]
      }
    }
  ]
};
