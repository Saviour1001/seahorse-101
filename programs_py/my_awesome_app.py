from seahorse.prelude import *

# This is your program's public key and it will update
# automatically when you build the project.
declare_id('BAv1R7yGTcga43cjmsHMcZsXTsLTxdK4UT4nMg8qzhTh')


class User(Account):
    owner: Pubkey
    note_count: u8
    last_note: u8


class Note(Account):
    owner: Pubkey
    index: u8
    title: str
    content: str


@instruction
def init_user(owner: Signer, user: Empty[User]):
    user = user.init(
        payer=owner,
        seeds=['user', owner]
    )
    user.owner = owner.key()
    user.note_count = 0
    user.last_note = 0


@instruction
def create_note(owner: Signer, user: User, note: Empty[Note], title: str, content: str):
    note = note.init(
        payer=owner,
        seeds=['note', owner, user.note_count]
    )
    note.owner = owner.key()
    note.index = user.note_count
    note.title = title
    note.content = content
    user.note_count += 1
    user.last_note = note.index


@instruction
def update_note(owner: Signer, noteIndex: u8, note: Note, title: str, content: str):
    assert note.owner == owner.key(), 'You are not the owner of this note'
    note.title = title
    note.content = content


@instruction
def delete_note(owner: Signer, note: Note):
    assert note.owner == owner.key(), 'You are not the owner of this note'
    note.content = ""
    note.title = ""
