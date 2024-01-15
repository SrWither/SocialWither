"""user and profile

Migration ID: 4a8cbf37f514
Revises: 
Creation Date: 2023-12-31 19:43:39.760824

"""

from emmett.orm import migrations


class Migration(migrations.Migration):
    revision = '4a8cbf37f514'
    revises = None

    def up(self):
        self.create_table(
            'users',
            migrations.Column('id', 'id'),
            migrations.Column('email', 'text'),
            migrations.Column('password', 'text'),
            primary_keys=['id'])
        self.create_index('users_widx__email_unique', 'users', ['email'], expressions=[], unique=True)
        self.create_table(
            'profiles',
            migrations.Column('id', 'id'),
            migrations.Column('username', 'string', length=512),
            migrations.Column('avatar', 'text'),
            migrations.Column('user_id', 'string', length=512),
            migrations.Column('bio', 'text'),
            primary_keys=['id'])
        self.create_index('profiles_widx__username_unique', 'profiles', ['username'], expressions=[], unique=True)

    def down(self):
        self.drop_index('profiles_widx__username_unique', 'profiles')
        self.drop_table('profiles')
        self.drop_index('users_widx__email_unique', 'users')
        self.drop_table('users')
