# monsatan-defacing

## Description

In this track, the participant need to perform a supply-chain by pushing a malicious dependency to modify the website. 
This will require interaction with a CI/CD pipeline in order to perform that supply-chain attack.

## Theme

The vigilante group wants to display their manifesto on the company's website.

## NOTES
In case it is needed during QA/debugging, the Administrator's GitLab account is `root:Ci6E6o3mUK1P8Ct`.
The root account also has this API token: `glpat-hJv-G9-aFa2dnlfm-GfdWW86MQp1OjEH.01.0w0m1mowv
rm2JNEZbgYH/WhpeFrr2vn9eWrw0dBlulOqf2J7v90g=`

## Solutions

### Flag 1
When viewing the source code of the website, the HTML has commented code that is a pipeline badge. When going to the 
GitLab repo that is public, the flag is in the README

### Flag 2
The commit history of the repo has a commit where a file with an access token was added and then removed. The flag is 
also in that file.

### Flag 3 (bonus)
After cloning the repo using the token found in flag 2, the participant can see that the CI configuration can't be 
modified, but a specific "legacy" file is still checked and executed. Participants can commit their own file and have 
the CI execute it. The flag is found by runnig the command "print_flag" which is part of the other "monsatan" commands 
that are pre-installed on the CI runner.

### Flag 4
The flag is in the environment of the CI job, but is masked. Participants needs to find a way to extract the masked 
value. The environment also contains an API key to access a dart repository used by the website compilation.

### Flag 5 (bonus)
Found in the security advisory of a test package in the dart registry.

### Flag 6
After uploading your own dependency to the dart registry, trigger a rebuild of the main branch using the endpoint in the
README. Once the manifesto is detected o nthe website, the flag is given
