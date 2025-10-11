<!--
title: Project: DB Client - UI and Name Thoughts
desc: No more Iced, thanks Rust GUI Survey
date: 2025-10-10
-->
Let's see how this looks without a big header. I've put a bit of time into the DB Client. Mostly stressing about which UI framework I'm going to use.

## So many GUI Libraries!
I was initially going to use [Iced](https://book.iced.rs/), but I wasn't completely sold. Before I even got into the docs I ended up seeing [Jonathan Kelley's recent presentation on Dioxus](https://www.youtube.com/watch?v=Kl90J5RmPxY). I really liked the tooling around it, his general philosophy, and desire to improve the entire Rust ecosystem.

I also considered if I should just make my UI from scratch like Zed did, or maybe I should just use Zed's now released UI crate, [gpui](https://www.gpui.rs/).

You see, I'm actually most stoked about Dioxus, but my biggest hangup is that it's just more HTML and CSS. An eternally doomed web dev is what I am. I specifically want this to be a desktop app, so it feels weird to use a web technology. On the other hand, it feels strangely responsible of me? Like it has the best chance to set me up for success.

Tauri is also there... not sure what to think about it.
## Rust GUI Surveey 2025
Enter [boringcactus' Rust GUI Survey](https://www.boringcactus.com/2025/04/13/2025-survey-of-rust-gui-libraries.html). A great resource that pretty much gave me the final push I needed to settle on Dioxus. I'd also be interested in flexing into [[Freya]], but for now I'll just explore Dioxus. I've made my peace with using web tech and Diet Electron, add some rum and I'm good to go.

Still interested in exploring other UI options in the future, but for now I'm gonna go ahead and commit.

## A Name!
I still don't have a committed name for the client, but the current frontrunner is `dbrs` — pronounced De Beers — do you think I'll get sued? Runner up is `dbeerus` for a DBZ nod. I'm sure I'll think of something better eventually.
