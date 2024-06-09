#!/bin/bash

curl "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key=$GOOGLE_API_KEY" \
-H "Content-Type: application/json" \
-d '{
        "contents": [
        {
            "role": "user",
            "parts": [
                {
                    "text": "Which theaters in Mountain View show Barbie movie?"
                }
            ]
        }
    ]
}'