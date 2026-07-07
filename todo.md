عالیه! از اینجا به بعد پروژه واقعاً هیجان‌انگیز می‌شود. Stageهای 5 تا 10 همان جایی هستند که کم‌کم از یک پنجره‌ی خالی به چیزی شبیه Wolfenstein و بعد DOOM می‌رسی.

---

# Stage 5/80

# Cast Your First Ray

**Difficulty:** ⭐⭐⭐⭐☆

**Time:** 2-4 ساعت

---

## Goal

اولین Ray را شلیک کن.

فعلاً فقط **یک Ray** از وسط صفحه.

اگر به دیوار برخورد کرد، روی ترمینال بنویس:

```text
Hit!
Distance: 4.37
```

اگر برخورد نکرد:

```text
Miss!
```

---

## World

فعلاً نقشه را Hardcode کن.

```text
##########
#........#
#........#
#...P....#
#........#
#........#
##########
```

* `#` = دیوار
* `.` = فضای خالی
* `P` = بازیکن

---

## Restrictions

از هیچ Physics Engine استفاده نکن.

---

## What You'll Learn

* Ray
* Ray Origin
* Ray Direction
* Grid Traversal
* برخورد Ray با Map

---

## Bonus

با چرخاندن دوربین، جهت Ray هم تغییر کند.

---

# Stage 6/80

# DDA Algorithm

**Difficulty:** ⭐⭐⭐⭐⭐

**Time:** 3-5 ساعت

---

## Goal

به جای حرکت میلی‌متری Ray، الگوریتم **DDA (Digital Differential Analyzer)** را پیاده کن.

Ray باید خانه‌به‌خانه روی Grid حرکت کند.

مثلاً:

```text
Player

↓

□ → □ → □ → █
```

---

## Restrictions

حق نداری Ray را این‌طوری حرکت بدهی:

```rust
pos += dir * 0.001;
```

این روش بسیار کند است.

---

## Expected Output

در حالت Debug اطلاعاتی مثل این چاپ شود:

```text
Step X
Map (4,5)

Step Y
Map (5,5)

Step X
Map (6,5)

Hit Wall
```

---

## What You'll Learn

* DDA
* deltaDist
* sideDist
* stepX
* stepY

---

## Bonus

مشخص کن برخورد از کدام سمت بوده است:

```text
North Wall
```

یا

```text
East Wall
```

---

# Stage 7/80

# Draw Your First Wall

**Difficulty:** ⭐⭐⭐⭐⭐

**Time:** 2-4 ساعت

---

## Goal

دیگر فقط یک Ray نداشته باش.

برای **هر ستون صفحه** یک Ray شلیک کن.

اگر عرض پنجره 960 پیکسل است، تقریباً 960 Ray خواهی داشت.

---

## Render

فعلاً فقط دیوارهای تک‌رنگ.

مثلاً:

```text
|||||||||||||||||
|||||||||||||||||
|||||||||||||||||
|||||||||||||||||
```

---

## Height Formula

هرچه دیوار دورتر باشد، کوتاه‌تر رسم شود.

از رابطه‌ی معروف استفاده کن:

```text
wallHeight = screenHeight / distance
```

---

## What You'll Learn

* Perspective Projection
* Distance Scaling
* Screen Space

---

## Bonus

اگر فاصله نصف شود، ارتفاع تقریباً دو برابر شود.

---

# Stage 8/80

# Fix the Fish-Eye Effect

**Difficulty:** ⭐⭐⭐⭐☆

**Time:** 1-2 ساعت

---

## Problem

اگر فقط فاصله‌ی Ray را استفاده کنی، دیوارها خمیده به نظر می‌رسند.

---

## Goal

فاصله را اصلاح کن تا دیوار صاف دیده شود.

به جای فاصله‌ی خام، از فاصله‌ی عمود بر صفحه‌ی دوربین استفاده کن.

---

## Expected Result

قبل:

```text
(
```

دیوار خمیده است.

بعد:

```text
|
```

دیوار کاملاً صاف است.

---

## What You'll Learn

* Camera Plane
* Perpendicular Distance
* Fish-eye Correction

---

## Bonus

توضیح بده چرا این اصلاح لازم است.

---

# Stage 9/80

# Add Colors

**Difficulty:** ⭐⭐☆☆☆

**Time:** 30-60 دقیقه

---

## Goal

دیوارها دیگر هم‌رنگ نباشند.

مثلاً:

* دیوارهای عمودی: آبی
* دیوارهای افقی: قرمز

یا:

* شمال: قرمز
* جنوب: سبز
* شرق: آبی
* غرب: زرد

---

## Bonus

دیوارهای دورتر را کمی تیره‌تر رسم کن.

---

## What You'll Learn

* Shading
* Surface Orientation
* Basic Lighting

---

# Stage 10/80

# Texture Mapping

**Difficulty:** ⭐⭐⭐⭐⭐

**Time:** 5-8 ساعت

---

## Goal

به جای رنگ ساده، دیوارها بافت (Texture) داشته باشند.

---

## Texture

فعلاً از یک تصویر 64×64 استفاده کن.

مثلاً:

```text
brick.png
```

---

## Requirements

برای هر Ray مشخص کن:

* به کدام دیوار برخورد کرده است.
* دقیقاً در چه نقطه‌ای از دیوار برخورد کرده است.
* کدام ستون از Texture باید رسم شود.

---

## Restrictions

هیچ Scaling آماده‌ای از کتابخانه‌ها استفاده نکن؛ خودت ستون‌های Texture را نمونه‌برداری (Sample) کن.

---

## What You'll Learn

* Texture Coordinates (UV)
* Wall Hit Position
* Texture Sampling
* Pixel Mapping

---

## Bonus

اگر از سمت چپ یا راست به دیوار نگاه می‌کنی، Texture برعکس نمایش داده نشود.

---

### بعد از Stage 10 چه خواهی داشت؟

اگر این مراحل را کامل انجام دهی، نتیجه چیزی شبیه این خواهد بود:

* ✅ حرکت در محیط
* ✅ Raycasting کامل
* ✅ دیوارهای سه‌بعدی
* ✅ اصلاح Fish-eye
* ✅ سایه‌زنی ساده
* ✅ دیوارهای تکسچرشده

در این نقطه، موتور تو از نظر رندر بسیار شبیه نسخه‌های اولیه‌ی **DOOM** و **Wolfenstein 3D** خواهد بود. بعد از آن وارد بخش‌های جذاب‌تر مثل **Floor Casting، Ceiling Casting، Sprite Rendering، Doors و در نهایت خواندن فایل‌های WAD** می‌شویم.
