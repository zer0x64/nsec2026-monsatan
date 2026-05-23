# monsatan-checkmate

## Description
In this track, you need to find the target profile and force a match with them to dump their information.

## Theme
For this challenge, the participants needs to hack into the Checkmate account of a Monsatan local representative. Checkmate is a dating app for chess enthusiats.

## Solutions

### Flag 1
- Create a user
- Swipe until you find the target. Click on their name to find their profile.
- The frontend gets the password hash and salt because the API returns the full DB model.
- Use that API to dump your own hash and use that to identify the parameters.
- You can use pbkdf2-identifier (`cargo install pbkdf2-identifier-cli`) to do this, but it's pretty easy to code by yourself if you understand PBKDF2. Basically, you check for a match at every iterations for a given primitive. You can run all primitives in parallel.
- Once you've identified the parameters, use this and the target's password hash to bruteforce it locally using rockyou.txt. For QA, the password is "clearfork4".
- The hashcat command to bruteforce the password: `hashcat -m 12100 -a 0 hash.txt --wordlist rockyou.txt`. The format of `hash.txt` is `sha512:1623:FCdNj2Ds4xltNHXpKhB3yA==:9Me0oGCiNvZb9eO85D0JZivzgBk1reME7Dl0/txcklU=`, or `sha512:1623:salt:hash`
- Once you've bruteforced the password, log into the target's account and swipe your own account. Go back to your account to swipe them back and go "chat" to them. The flag will be in the right text box in the location section.
