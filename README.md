# ImPass - a simple image-based password manager

ImPass is a simple and effective password manager that stores your passords as images.

## Why use it?
If you have a bunch of passwords or secrets and do __not__ believe yourself to remember them all, 
or any of the password mangers available - well, this is the tool for you!

ImPass stores your passwords _locally_, in a folder specified by the user. 
All passwords are stored as images (currently only PNG format is supported). 

The local nature of the application makes sure that your secrets are safe, as you can control __*ALL*__ 
parts of the application - from the encryption/decryption parameters, to password-protecting your secrets (within the image) AND
the way your images are stored. 

## How does it work
ImPass can be used as a simple command-line or visual tool (coming in the future).

The application stores secrets in images (ones that you provide) and is the __only__ point that can retrieve said secrets back from the images.
Each configuration is special to your local machine, meaning that only you can decrypt the encrypted information.

And yes, this means that if you store your secrets and then change/loose the configuration of the application, your secret will not be retrievable.

### Secrets and security
When creating a new secret you cam provide a password.
The password is completely optional, but provides a simple security mechanism - it is stored within your secret.
When decoding your secret, you will be prompted to enter the password that was encoded with it. 
The user can set a number of unsuccessful tries for the password, meaning that if you get it wrong X amount of times - we'll delete the image, and all secrets within it.

### More docs to come with development

