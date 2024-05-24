import requests

GENAI_API_DISCOVERY_URL= "https://generativelanguage.googleapis.com/$discovery/rest?version=v1beta&key="
GOOGLE_API_KEY = "AIxxxx"
url=f"{GENAI_API_DISCOVERY_URL}{GOOGLE_API_KEY}"

response = requests.get(url)
print(response.text)