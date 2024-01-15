from emmett.validators import Validator
from emmett.orm import Model, Field
from argon2 import PasswordHasher

class Argon2Validator(Validator):
    ph = PasswordHasher()
    message = "Invalid value"

    def __call__(self, value):
        hashed_password = self.ph.hash(value)
        try:
            self.ph.verify(hashed_password, value)
            return hashed_password, None
        except ValueError:
            return value, self.message


class User(Model):
    email = Field().text(unique=True)
    password = Field().text(validation=Argon2Validator())

class Profile(Model):
    username = Field(unique=True)
    avatar = Field().text()
    user_id = Field()
    bio = Field.text()
