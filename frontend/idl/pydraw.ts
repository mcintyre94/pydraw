export type Pydraw = {
  "version": "0.1.0",
  "name": "pydraw",
  "instructions": [
    {
      "name": "createPixel",
      "accounts": [
        {
          "name": "pixel",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "pixel"
              },
              {
                "kind": "arg",
                "type": "u8",
                "path": "pos_x"
              },
              {
                "kind": "arg",
                "type": "u8",
                "path": "pos_y"
              }
            ]
          }
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "posX",
          "type": "u8"
        },
        {
          "name": "posY",
          "type": "u8"
        },
        {
          "name": "initColR",
          "type": "u8"
        },
        {
          "name": "initColG",
          "type": "u8"
        },
        {
          "name": "initColB",
          "type": "u8"
        }
      ]
    },
    {
      "name": "updatePixel",
      "accounts": [
        {
          "name": "pixel",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "newColR",
          "type": "u8"
        },
        {
          "name": "newColG",
          "type": "u8"
        },
        {
          "name": "newColB",
          "type": "u8"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "pixel",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "posX",
            "type": "u8"
          },
          {
            "name": "posY",
            "type": "u8"
          },
          {
            "name": "colR",
            "type": "u8"
          },
          {
            "name": "colG",
            "type": "u8"
          },
          {
            "name": "colB",
            "type": "u8"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "E000",
      "msg": "The given X co-ordinate is not between 0-99"
    },
    {
      "code": 6001,
      "name": "E001",
      "msg": "The given Y co-ordinate is not between 0-99"
    },
    {
      "code": 6002,
      "name": "E002",
      "msg": "The given red colour is not between 0-255"
    },
    {
      "code": 6003,
      "name": "E003",
      "msg": "The given green colour is not between 0-255"
    },
    {
      "code": 6004,
      "name": "E004",
      "msg": "The given blue colour is not between 0-255"
    }
  ]
};

export const IDL: Pydraw = {
  "version": "0.1.0",
  "name": "pydraw",
  "instructions": [
    {
      "name": "createPixel",
      "accounts": [
        {
          "name": "pixel",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "pixel"
              },
              {
                "kind": "arg",
                "type": "u8",
                "path": "pos_x"
              },
              {
                "kind": "arg",
                "type": "u8",
                "path": "pos_y"
              }
            ]
          }
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "posX",
          "type": "u8"
        },
        {
          "name": "posY",
          "type": "u8"
        },
        {
          "name": "initColR",
          "type": "u8"
        },
        {
          "name": "initColG",
          "type": "u8"
        },
        {
          "name": "initColB",
          "type": "u8"
        }
      ]
    },
    {
      "name": "updatePixel",
      "accounts": [
        {
          "name": "pixel",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "newColR",
          "type": "u8"
        },
        {
          "name": "newColG",
          "type": "u8"
        },
        {
          "name": "newColB",
          "type": "u8"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "pixel",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "posX",
            "type": "u8"
          },
          {
            "name": "posY",
            "type": "u8"
          },
          {
            "name": "colR",
            "type": "u8"
          },
          {
            "name": "colG",
            "type": "u8"
          },
          {
            "name": "colB",
            "type": "u8"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "E000",
      "msg": "The given X co-ordinate is not between 0-99"
    },
    {
      "code": 6001,
      "name": "E001",
      "msg": "The given Y co-ordinate is not between 0-99"
    },
    {
      "code": 6002,
      "name": "E002",
      "msg": "The given red colour is not between 0-255"
    },
    {
      "code": 6003,
      "name": "E003",
      "msg": "The given green colour is not between 0-255"
    },
    {
      "code": 6004,
      "name": "E004",
      "msg": "The given blue colour is not between 0-255"
    }
  ]
};
