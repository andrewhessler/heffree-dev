<!--
title: Project: Personal Site + Blog
desc: A rundown of what went into getting this blog up and running
date: 2025-10-05
-->
# Personal Site + Blog
I created this blog for a few reasons: 
- I like writing (and typing). 
- I'm interested in participating in the conversation, being apart of the general programming zeitgeist. 
- I'd love to get feedback on both my programming and writing.
- It's really a place for me to rubber duck and debrief, but I hope it can entertain and maybe even help others.

The rest of the personal site is there to direct people to my socials and stuff. I thought I would end up hosting some webapps on the same domain, but I'm not sure that'll be happening. On the same Linode pod, I host another personal site that my wife and I use for little throwaway tools.

Anyway, let's get into it!

## Technical Evolution

### The Stack
I've been using a pretty consistent stack since I started [Planning Poker](./project-planning-poker.html) &mdash; which I'll do a post on in a little bit. That stack is:
- Cloudflare domain (DNS only)
- Linode pod
- Nginx proxy
- Rust webserver using axum
- React client (embedded in the webserver executable)
- Sqlite DB

And that's how this blog started as well. My Rust webserver serves both API routes and the files for the React client.

With this being just a simple blog, I immediately wanted to drop the React part and I have no current intentions of using a DB. I figured I'd just host some static files from my webserver and leave it at that. 

I also don't really need any APIs, so why have the webserver at all? Nginx is a perfectly acceptable webserver, so I'll just use that. I may regret this a little as I think tracing might be a bit simpler if I had a personal webserver to tweak programmatically, but I'm also not pressed about it.

### Where's the Rust?
But how can I live up to the Rust developer stereotypes if I don't try and shoehorn Rust into every project I ever touch?

Before I even got to work on my first real post, I was already getting frustrated with continuously find-and-replacing the nav — and that's on like 7 files. I deleted and readded the blog link every other day depending on how silly I felt having only a placeholder entry.

I've been following [Armin Ronacher](https://lucumr.pocoo.org/) recently to observe his LLM playground. I'm pretty bearish on LLMs (from my personal experiences with them), but he seems like the person to keep an eye on if any breakthroughs do happen. Tangentially, I also know one of the things he's well-known for is the templating engine Jinja2 &mdash; but I don't want to use Python, I want to use Rust. I did browse his [public source personal site](https://github.com/mitsuhiko/lucumr) for inspiration, though.

I also don't want to make my own templating engine, that's just not what I want to work on right now. Instead I reached for [handlebars](https://handlebarsjs.com/), but the Rust [manifestation](https://crates.io/crates/handlebars). Handlebars is something I've used, I just really can't remember when, which means it must've been at my first job or in college. All I remember is a lot of mustache discussion, which I'm incapable of growing.

Thus the webserver transformed into a static site generator.
### Obsidian
Both the aesthetic design and technical implementation are motivated by my use of Obsidian. I wanted most of my site to be able to exist as markdown files and be rendered to html. There's a [rust crate](https://crates.io/crates/markdown) for that!

I've come to peace with my wanton importing of dependencies. I'm up to a total of 6 (and of course what they rely on); I'd argue `anyhow`, `serde`, and `walkdir` hardly count, but oh well. I'm a little sad about the potentially missed learning opportunities, but I can always go back. I'm going to continue to focus on the things that I see value in.

### Implementation Wrap-up
That's where I'm at now:
- Cloudflare domain (DNS Only)
- Linode pod
- Nginx webserver
- Rust static site generator
- a dist folder

The public source can be found [here](https://github.com/andrewhessler/heffree-dev). I'll update the README with the nginx config. The nginx config started out pretty simple, but definitely got simpler after I stopped using it as a proxy.

I do some silly things with the generator. It's not structured well, but that's the benefit of building for yourself. I do multiple layers of template rendering, which interestingly is kind of similar to what we do at my current job with our Helm templating.

It was really satisfying to recognize some obvious over-engineering. And that over-engineering might even be valid for someone else's vision, it's really just about the tradeoffs. I might be over-engineered for someone else's vision. I just struggle to determine if any implementation is correct. Are all working implementations correct? I think yes. I also think depending on the context there can be **more correct** implementations.

I guess I feel better saying it was really satisfying to recognize such a distinct alternative architecture.

## Design
As I mentioned earlier, one of my driving inspirations was Obsidian. I like the compressed area in the middle and it fits mobile well. I personally like the styling I have in Obsidian, but I think some people might see it as overkill. I may start to leak more of it into the site as time goes on.

For my second inspiration, I decided to use some toned down colors from my terminal colorschemes. The background of the site is the exact color of my terminal background, but the orange and cyan are adjacent to my tmux window split colors (X11 orange and cyan). They're pretty Portal inspired, but not the exact hexcodes.

Additional inspiration includes:
- My nav looks egregiously similar to [Colton Voege's](https://colton.dev/), but maybe we can all just agree there's only so many ways to layout 5 elements and a horizontal rule/bottom border...
- [Armin's blog](https://lucumr.pocoo.org/), as I mentioned.
- I noticed Martin Fowler calls [his blog](https://martinfowler.com/bliki/) a "bliki", so I named my blog entry index "blikidex" as an homage. My wife informed me "bliky" is also slang for a gun nowadays, so that's fun. My actual content continues to reference it as a blog.
- I also really like [Nolen Royalty's](https://eieio.games/blog/) EIEIO blog, I intend to eventually do something similar to him where my less relevant thoughts can be tossed off to the side; maybe anything that exists in parentheses.
- I enjoy the simplicity of [Nikhil Suresh's blog](https://ludic.mataroa.blog/) and I also now realize at one point my blog index looked identical to his, unintentionally.
- Honorable mentions: [Yossi Kreinin's blog](https://www.yosefk.com/blog/) and [Yoshua Wuyts' blog](https://blog.yoshuawuyts.com/)

I'm sure there is plenty of iteration to come, but for now this is where I'm at.
