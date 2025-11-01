<!--
title: Project: Beebfam Site
desc: A place for me and mine
date: 2025-10-30
author: Andrew Hessler
-->
# Project Beebfam Site
I'm pretty interested in making a little hub app for my wife and I to use. 
Specifically a mobile app, but I don't have a personal Mac to sign things for our iPhones.

So instead I just made a webapp: [beebfam.org](beebfam.org)

No need to go into the origin of the name, [repo here](https://github.com/andrewhessler/beebfam).

So far I've made a few apps that we use regularly and I intend to add more; probably some home automation when we finally own a home.

## First app - Mario Picker
My wife and I have been playing quite a bit of Mario Kart World. Watching our favorite streamer Shortcat to learn all the shortcuts.

Mario Kart World has this weird quirk where on top of the 30-some courses, there's also 200 intermission tracks.
Not many people like the volume of these intermission tracks, they're generally less interesting than the main lap-based races.

So, most people play in Versus mode and just choose random courses to play, since all the prix (plural of prix) automatically include intermission tracks.

All this to say, we were using a wheel spinning app and my wife didn't trust it was random so I made us a quick 4-race randomizer in React.

## Second app - Chore Kanban
The first app I made with a backend was Chore Kanban. This is just a virtualization of something we used to do on our fridge with magnets until we ran out of steam.
Now it's mostly automated and we just have to hit the buttons and do what it tells us!

If any of the apps have any persistence, they're backed by an axum webserver and a sqlite DB. Same stack as mentioned in my [personal site post](./project-personal-site-1.md).

The data design of this app is just ridiculous;
I did the old form of vibecoding (LLM-less) and just started adding fields as I needed them.

I'll try to describe the design from memory. 
Each chore has a `last_completed_at` datetime and a `cadence`, the due date is calculated as the `cadence` plus the... I lied, I'm going to look.

Okay, back! So `last_completed_at` does exist, but instead the due date is calculated by adding `frequency_hours` (hours!).
Now if `on_cadence` — which is a boolean — is enabled, then `last_completed_at` will only change by `frequency_hours`, so it can be toggled between due/upcoming,
but will always be due on the same day.

If `on_cadence` is false, these are due `frequency_hours` from `last_completed_at` and `last_completed_at` is set to the time the chore is actually completed.

If `frequency_hours` is null then the chore is considered "Ad Hoc" and it just tracks `last_completed_at`, we must mark it due and upcoming, I think it nulls `last_completed_at` to be due.

## Third app - Andrew Inbox
The third app is just a simple input, simple table. Add entries through the input, delete them by tapping on them.

We've already had a kerfuffle with this one where my wife (Melissa, by the way) added a note for me and I thought "What? Why did I add this note?" and deleted it.

Now I know she knows she can use it lol.

## Fourth app - Shopping List
This one got out of hand with the naming, sometimes called Supply List, Grocery List, Shopping List, it's all over the place.
It's just a list of things we need to buy or restock on.

A little more involved schema, but nothing crazy. Got `quantity` and `category` in addition to `name`. If you re-enter an item it just updates it's related values.
In fact it's the intended way to update the quantity to something else.
I want to add something that automatically rematches the category so we don't accidentally move that around.

## Auth
I hope my auth is full-proof enough. Can definitely be cracked if you get into my pod, but you'll never guess my password!

Password and cookie key are stored on the server, cookie lives for a year — don't social engineer Melissa, please — or even worse, me.

Technically the fifth app is the root that links to all the other apps and handles login.
It then shares the cookie key with the other apps which block all their mutations against the key.
All the reads still work without auth because I like to share the site and no one is even going to look at *this* site... meh,
hopefully I don't regret that anytime soon.

## Structure
I originally had a repo per webapp but moved them to a monorepo for convenience.
Each app is hosted on a subdomain because I didn't want the apps themselves to care about what nested route was theirs to own.

## Small Thoughts
As you can see, they aren't the most well thought out or robust apps, but they work just fine and we're aware of their idiosyncrasies.
It's really nice to be able to build for yourself. A lot less pressure than creating a product.

I'm sure I'll add plenty more and each time I'll need to setup a systemd service, Cloudflare DNS, and nginx config.
It's not the most horrible overhead, but I'm sure I could simplify it if I cared to try.
