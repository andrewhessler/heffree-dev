<!--
title: Project: DB Client (Pre-natal)
desc: Really quick thoughts on my next project
date: 2025-10-07
-->
# DB Client
Finally! This is what this blogging mini-marathon has all been for. Saying goodbye to [Planning Poker](./project-planning-poker-post.html) and welcoming in the DB Client.

I've serendipitously recently watched Gavin King talk about Hibernate and the idea of a Data Access Engineer. Somewhere in between an Application Developer and a DBA.

I've been especiialy interested in data and DB operations most of my career. Maybe because my first job involved migrating between two databases, I had a lot I needed to learn to be effective.

So, my goal is to make a DB Client. Azure Data Studio is going away, maybe I can fill the void. Probably not since this will probably take me 3+ years, but at least it's something I can be proud to work on as a hobby project. It will hopefully present plenty of interesting learning opportunities.

This is just preliminary thoughts and I'm lacking on extremely short blog posts, so let's keep it short and let me get to work.

## Plan
I think I need 3 major parts: editor, DB connection, and result handling.

I'm considering Tauri and Iced for UI purposes, not sure. And I've been reading software like this usually uses a microkernel architecture so plugins can be added, which is something I'm pretty sure I want.

Anyway, gonna get started with a small prototype. Type text, connect to a DB, display results. I'll probably start by targeting SQL Server or sqlite.

I don't actually have a lot of DB Administration experience, but I'd love to some day rival SQL Server Management Studio. Probably impossible, but that's my loftiest goal.

My most achievable goal is to just use this once at my job for executing a query.

I'll also need to think of a good name.

That's it! Here I go...
