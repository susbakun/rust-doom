Stage 14 — Player System

الان احتمالاً فقط position داری.

اضافه کن:

struct Player {
    pos: Vec2,
    dir: Vec2,
    plane: Vec2,
    speed: f64,
    rotation_speed: f64,
}

قابلیت‌ها:

forward
backward
strafe
rotate
Stage 15 — Collision

الان اگر حرکت کنی:

########
#      #
#  P   #
########

می‌توانی بروی داخل دیوار.

حلش:

new_x = player.x + velocity.x

if !world.is_wall(new_x, player.y) {
    player.x = new_x;
}

و جدا برای Y.

این باعث sliding روی دیوار می‌شود.

بعد از این وارد بخش‌های Doom-like می‌شویم:

Stage 16 — ارتفاع‌ها (خیلی مهم)

دیگر:

#

فقط دیوار نیست.

می‌شود:

struct Sector {
    floor: f64,
    ceiling: f64,
}

مثلاً:

room A

ceiling 4
---------
floor 0


room B

ceiling 8
---------
floor 2

اینجا پله و سکو می‌سازی.

Stage 17 — Sector Map

تغییر بزرگ:

از:

[
 [#,.,.,#],
 [#,.,.,#]
]

می‌رویم به:

Sector {
    walls: Vec<Wall>,
    floor_height,
    ceiling_height,
}
Stage 18 — Portals

دو اتاق که به هم وصل می‌شوند.

مثل DOOM:

Room A

########
#      #
#      #==== Room B
########

دیگر دیوار فقط یک دیوار نیست، ممکن است پنجره به Sector بعدی باشد.

Stage 19 — Objects / Sprites
دشمن
آیتم
اسلحه
Stage 20 — Visibility Optimization

اینجا دیگر شبیه موتورهای قدیمی می‌شوی:

BSP یا Portal traversal
حذف چیزهای خارج دید
