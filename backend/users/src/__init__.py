import argon2
from emmett import App
from emmett.orm import Database
from emmett_rest import REST
import jwt
from .models.users import User, Profile
from emmett.tools import service
from argon2 import PasswordHasher
from emmett import response, request, Pipe
from dotenv import load_dotenv
from emmett.ctx import current
import os

load_dotenv()

DATABASEURL = os.getenv("DATABASEURL")
SECRETKEY = os.getenv("SECRETKEY")

class CORS(Pipe):
    async def open(self):
        response.headers["Access-Control-Allow-Origin"] = "*"
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "OPTIONS, GET, POST, PUT, PATCH, DELETE"
        response.headers["Access-Control-Allow-Headers"] = "Origin, Content-Type, Accept, Authorization"


app = App(__name__)
app.config.db.uri = DATABASEURL
rest = app.use_extension(REST)

ph = PasswordHasher()

db = Database(app)
db.define_models(User, Profile)

app.pipeline = [db.pipe, CORS()]

users = app.rest_module(__name__, 'api_users', User, url_prefix='users', serialize_meta=False, list_envelope="users")
profiles = app.rest_module(__name__, 'api_profiles', Profile, url_prefix='profiles', serialize_meta=False, list_envelope="profiles")

@app.route('/<any:p>', methods='OPTIONS', pipeline=[CORS()])
def _options(p):
    return ""

@app.route("/getprofile/<int:user_id>", methods=["GET"])
@service.json
async def get_profile_by_user_id(user_id):
    profile = db(Profile.user_id == user_id).select().first()

    if profile:
        response.status = 200
        return profile.as_dict()
    else:
        response.status = 404
        return {'error': 'Profile not found'}

@app.route("/login", methods=["POST"])
@service.json
async def login():
    data = await request.body_params
    email = data.email
    password = data.password

    if not email or not password:
        response.status = 400
        return {'error': 'Email and password are required'}

    user = db(User.email == email).select().first()

    if user:
        try:
            ph.verify(user.password, password)
            response.status = 200
            return dict(user=user)
        except argon2.exceptions.VerifyMismatchError:
            response.status = 401
            return {'error': 'Invalid credentials'}
    else:
        response.status = 401
        return {'error': 'Invalid credentials'}

@app.route("/register", methods=["POST"])
@service.json
async def register():
    data = await request.body_params
    email = data.email
    password = data.password

    if not email or not password:
        response.status = 400
        return {'error': 'Email and password are required'}

    with db.connection():
        newuser = User.create(
            email=email,
            password=password
        )

        db.commit()

    user = User.get(newuser.id)

    if user:
        response.status = 200
        return user.as_dict()
    else:
        response.status = 500
        return {'error': 'Error creating user'}


@app.route("/create-profile", methods=["POST"])
@service.json
async def create_profile():
    data = await request.body_params
    username = data.username
    avatar = data.avatar
    bio = data.bio

    if not username or not avatar or not bio:
        response.status = 400
        return {'error': 'Username, avatar, and bio are required'}

    token = request.headers.get("Authorization")

    if not token:
        response.status = 401
        return {"error": "No token"}

    puretoken = token[len("Bearer "):] if token.startswith("Bearer ") else token

    try:
        decoded_data = jwt.decode(jwt=puretoken,
                                key=SECRETKEY,
                                algorithms=["HS256"])

        with db.connection():
            newprofile = Profile.create(
                username=username,
                avatar=avatar,
                user_id=decoded_data["user_id"],
                bio=bio
            )

            db.commit()

        profile = Profile.get(newprofile.id)

        if profile:
            response.status = 200
            return profile.as_dict()
        else:
            response.status = 500
            return {'error': 'Error creating profile'}

    except jwt.exceptions.PyJWTError as e:
        response.status = 401
        return {'error': f'{e}'}


@profiles.update()
async def profile_edit(dbset, rid):
    token = request.headers.get("Authorization")

    # check token
    if not token:
        response.status = 401
        return {"error": "No token"}

    puretoken = token[len("Bearer "):] if token.startswith("Bearer ") else token

    try:
        # decode token
        decoded_data = jwt.decode(jwt=puretoken,
                                key=SECRETKEY,
                                algorithms=["HS256"])

        # get profile
        attrs = await profiles.parse_params()
        row = dbset.where(Profile.id == rid).select().first()

        # check user_id
        if not row.user_id == decoded_data["user_id"]:
            response.status = 403
            return {"error": "You cannot edit this profile"}

        # check if profile exists
        if not row:
            response.status = 404
            return {"error", "Profile not found"}

        # update profile
        current._dbvalidation_record_id_ = row.id
        row.update(**attrs, user_id=row.user_id)

        # check if the profile has been updated
        if not row.save():
            response.status = 422
            return {"error": "Changes could not be saved"}

        return profiles.serialize_one(row)

    except jwt.exceptions.PyJWTError as e:
        # token error response
        response.status = 401
        return {"error": f"{e}"}

@profiles.delete()
async def profile_del(dbset, rid):
    token = request.headers.get("Authorization")

    # check token
    if not token:
        response.status = 401
        return {"error": "No token"}

    puretoken = token[len("Bearer "):] if token.startswith("Bearer ") else token

    try:
        # decode token
        decoded_data = jwt.decode(jwt=puretoken,
                                  key=SECRETKEY,
                                  algorithms=["HS256"])

        # get profile
        row = dbset.where(Profile.id == rid).select().first()

        # check if profile exists
        if not row:
            response.status = 404
            return {"error": "Profile not found"}

        # check user_id
        if not row.user_id == decoded_data["user_id"]:
            response.status = 403
            return {"error": "You cannot delete this profile"}

        # delete profile
        if not row.destroy():
            response.status = 422
            return {"error": "Profile could not be deleted"}

        return {"message": "profile deleted successfully"}

    except jwt.exceptions.PyJWTError as e:
        # token error response
        response.status = 401
        return {"error": f"{e}"}

@users.update()
async def user_edit(dbset, rid):
    token = request.headers.get("Authorization")

    # check token
    if not token:
        response.status = 401
        return {"error": "No token"}

    puretoken = token[len("Bearer "):] if token.startswith("Bearer ") else token

    try:
        # decode token
        decoded_data = jwt.decode(jwt=puretoken,
                                key=SECRETKEY,
                                algorithms=["HS256"])

        # get user
        attrs = await users.parse_params()
        row = dbset.where(User.id == rid).select().first()

        # check user_id
        if not str(row.id) == decoded_data["user_id"]:
            response.status = 403
            return {"error": "You cannot edit this user"}

        # check if user exists
        if not row:
            response.status = 404
            return {"error", "User not found"}

        # update user
        current._dbvalidation_record_id_ = row.id
        row.update(**attrs)

        # check if the user has been updated
        if not row.save():
            response.status = 422
            return {"error": "Changes could not be saved"}

        return users.serialize_one(row)

    except jwt.exceptions.PyJWTError as e:
        # token error response
        response.status = 401
        return {"error": f"{e}"}


@users.delete()
async def user_del(dbset, rid):
    token = request.headers.get("Authorization")

    # check token
    if not token:
        response.status = 401
        return {"error": "No token"}

    puretoken = token[len("Bearer "):] if token.startswith("Bearer ") else token

    try:
        # decode token
        decoded_data = jwt.decode(jwt=puretoken,
                                  key=SECRETKEY,
                                  algorithms=["HS256"])

        # get profile
        row = dbset.where(User.id == rid).select().first()

        # check if profile exists
        if not row:
            response.status = 404
            return {"error": "User not found"}

        # check user_id
        if not str(row.id) == decoded_data["user_id"]:
            response.status = 403
            return {"error": "You cannot delete this profile"}

        # delete profile
        if not row.destroy():
            response.status = 422
            return {"error": "User could not be deleted"}

        return {"message": "user deleted successfully"}

    except jwt.exceptions.PyJWTError as e:
        # token error response
        response.status = 401
        return {"error": f"{e}"}


@profiles.on_404
def profiles_404err():
    return {'error': 'Profile not found'}

@users.on_404
def users_404err():
    return {'error': 'User not found'}
