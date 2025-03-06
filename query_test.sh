curl --request POST \
  --url https://flare.hypersync.xyz/query \
  --header 'Content-Type: application/json' \
  --data '{
    "from_block": 38381480,
"logs": [
		{
			"address": [
				"0xCEe8Cd002F151A536394E564b84076c41bBBcD4d"
			]
		}
	],
    "field_selection": {
        "block": [
            "number",
            "timestamp",
            "hash"
        ],
        "log": [
            "block_number",
            "log_index",
	    "to",
	    "from",
            "transaction_index",
            "data",
            "address",
            "topic0",
            "topic1",
            "topic2",
            "topic3"
        ],
        "transaction": [
            "block_number",
            "transaction_index",
            "hash",
            "from",
            "to",
            "value",
            "input"
        ]
    },
	"max_logs": 10  
}'
