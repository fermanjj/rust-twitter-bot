# Exposed Passwords Twitter Bot

### Basic Twitter Oauth2 steps

[Oauth2 Twitter Docs](https://developer.twitter.com/en/docs/authentication/oauth-2-0/user-access-token)

Following these steps got an Oauth2 access token

Hit [this URL](https://twitter.com/i/oauth2/authorize?response_type=code&client_id=CLIENT_ID&redirect_uri=REDIRECT_URI&scope=tweet.read%20tweet.write%20users.read%20follows.read%20offline.access&state=state&code_challenge=challenge&code_challenge_method=plain)

- CLIENT_ID Example: `M1M5R3BMVy13QmpScXkzTUt5OE46MTpjaQ`

- REDIRECT_URI Example: `https://2xdzlqwmizkelr7naddet3nqau0gpuzd.lambda-url.us-east-1.on.aws/`

Get the code from the query string params

- Example: `MHk2RzZVdTcyaEVwVFFuS1RRendfLXBGVl9OQUNGNlZPa0Iwa1BWeWFKSEliOjE2NjA4Njg5NjI4NDg6MToxOmFjOjF`

Next send a request like so

```
curl --location --request POST 'https://api.twitter.com/2/oauth2/token' \
--header 'Content-Type: application/x-www-form-urlencoded' \
--header 'Authorization: Basic BASE64ENCODED'\
--data-urlencode 'code=CODE' \
--data-urlencode 'grant_type=authorization_code' \
--data-urlencode 'redirect_uri=REDIRECT_URI' \
--data-urlencode 'code_verifier=challenge'
```

- In the auth header goes a base64 encoded `CLIENT_ID:CLIENT_SECRET`

From that response you should get this
```json
{
    "token_type": "bearer",
    "expires_in": 7200,
    "access_token": "ACCESS_TOKEN",
    "scope": "follows.read offline.access tweet.write users.read tweet.read",
    "refresh_token": "REFRESH_TOKEN"
}
```

Using the refresh token we can make a request like this to get another set of tokens

```
curl --location --request POST 'https://api.twitter.com/2/oauth2/token' \
--header 'Content-Type: application/x-www-form-urlencoded' \
--header 'Authorization: Basic BASE64ENCODED'\
--data-urlencode 'refresh_token=REFRESH_TOKEN'\
--data-urlencode 'grant_type=refresh_token' 
```

### Building and pushing Image
