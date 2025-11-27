<!--
title: I like TypeScript
desc: and I don't want to use Rust for my frontend
date: 2025-11-22
author: Andrew Hessler
-->
# I like TypeScript
It's pretty great. If anything because it shows me how valuable the compile-time guarantees of Rust are.
In turn, Rust shows me how to leverage the build-time guarantees of TypeScript.
I might even revisit C++ to start appreciating Rust from the other side — I'm getting into pretty serious zealotry here, though, so maybe later.
But also I like TypeScript because it's super flexible with a huge ecosystem. And most importantly, I'm pretty familiar with it so it's comfortable to use.

I like the idea of being fairly familiar with each of them and using them for what they're good at.
I think a solid foundation in both will help me leverage a lot of other tools/languages as well.
It's hard to define what types of programs Rust is good for and I'm not even sure you can say it's necessarily good for a webserver,
but a webserver is at least a pretty simple frontend for a system's logic — and I think encoding logic is something I *can* say Rust is good at.

What do I know, though? Anyway, just wanted to write a post to say I don't think I'll be continuing to use Dioxus for my DB Client project.
Going to give Tauri a try, I don't need to be trying the exotic pieces of a language and ecosystem that I'm still essentially a newcomer in 
— or I can at least give myself the leniency to bail out of the exotic bits I have tried.

```typescript
function testingSomeStuff() {
    console.log('hi bud');
}
```
Hey neat, got code blocks looking nice — totally worth the included JavaScript, though I guess the floodgates are open now.

Edit 2025-11-26: I was previously using [highlight.js](https://github.com/highlightjs/highlight.js), but why!? 
Seems weird to have the browser do it when I already know the contents and language type of the code block...

So instead I'm using [syntect](https://github.com/trishume/syntect). Not nearly as convenient, I had to download Sublime just to convert the `tmLanguage` syntax
file to a `sublime-syntax` file, but at least I get the effect pre-read. 
Don't even have to worry about any [FOUC](https://en.wikipedia.org/wiki/Flash_of_unstyled_content)s.
Don't have to give a FOUC... 
and the floodgates are closed again.
