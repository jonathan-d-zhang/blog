Build a Blog
74389

Why do I have a blog? How did I make it?

### Table of Contents
 - [Senior Project](#senior-project)
 - [Creating and Reading Articles](#creating-and-reading-articles)
 - [Using Docker](#using-docker)
 - [Images?](#images)

## Senior Project <a id="senior-project" class="anchor"></a>
At my high school, in order to graduate, each senior must complete a Senior Project with the help of a mentor. For 15 days, students do "an experiential exploration of a topic of interest to the individual student".

For my project, I wanted to make a website. Prior to making this blog, I had no experience with web development, so this was a nice way to see what it's like.

I decided to use Rust, because I haven't used it for an actual project yet. The backend framework I used was [rocket.rs](https://rocket.rs/). I used [handlebars.js](https://handlebarsjs.com/) for templating. It's currently hosted on AWS Lightsail Containers.

## Creating and Reading Articles <a id="creating-and-reading-articles" class="anchor"></a>
The first few days were spent creating the basic routes: reading articles, the home page, etc. These were relatively simple to make; `rocket.rs` made it quite easy.

The articles themselves are stored in a JSON file, along with some metadata. I considered using a database, but decided that since they would mostly be read-only, I didn't have to worry about atomicity issues.

The process of deploying new articles to the website was not as easy as I thought it would be. I went through a few iterations of how I wanted to get new articles onto the web server. First, I used a form on the website itself to upload articles directly by copying and pasting the text in. The problem with this method was that I had to worry about security, since I only want myself to be able to upload articles. It was also relatively unsophisticated, since I would have to manually back up the articles to git.

The next idea was to not upload through the website, but rather SSH into the web server and copy over the articles. This method bypasses the security issue of the previous method, which is great. The only problem is that you don't have SSH access into Lightsail Containers, so this method can't work.

The method I finally settled on was to simply create a new container with the new article inside, and push that to Lightsail. This method had none of the security issues of the first method, since I offload all secrets management to Github Secrets, and it could be entirely automated with GitHub Actions and Lightsail's CLI. The only annoying thing was writing a JSON string inline into a command, as shown by my commit history:

```plaintext
775ee90 Forgot quotes
2f10ff3 Another missing `}`
a567127 Forgot quotes
8917168 Need to actually use the variable
4552763 Missed a `}`
7043724 Apparently you can't use ":" in tags for aws
1be7843 Install lightsailctl and try again
df52a61 Try pushing to lightsail
7464a5c Actually load the docker image
9f180db Test if docker container builds properly
da8ecdf Test if docker container builds properly
9052818 Add service name
371dddc Maybe this will work?
d9c1018 Change workflow path to work on any change in articles/
9d38048 Add a test markdown file
4cc17b3 Test gh actions
```

On startup, the `articles/md` directory is checked for new markdown files. Any new files are compiled and added to `articles/json`.

Adding support for Markdown was quite trivial with the `pulldown-cmark` crate. It only took a few lines (and most of them were optional!).
```rust
let mut options = Options::empty();
options.insert(Options::ENABLE_STRIKETHROUGH);
options.insert(Options::ENABLE_TABLES);
let parser = Parser::new_ext(rest, options);
let mut output = String::new();
html::push_html(&mut output, parser);
```

Adding fonts and css was also relatively easy. I just used `rocket::fs::FileServer` to serve the files, and it Just Worked.
```rust
fn launch() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![index, get_article::article_page, get_article::articles],
        )
        .mount("/styles", FileServer::from(relative!("styles")))
        .mount("/fonts", FileServer::from(relative!("fonts")))
        .attach(Template::fairing())
}
```

## Using Docker <a id="using-docker" class="anchor"></a>

I had never used Docker before, so my first Dockerfiles were quite rudimentary.
```Dockerfile
FROM rust:1.54
COPY . .
RUN cargo build --release
CMD ["./target/release/blog"]
EXPOSE 80
```
This was bad for several reasons. First, this image has all the rust tooling needed to compile the project, which is around 5.97 GB 😬. Second, each time I ran `docker build`, everything had to be recompiled, including the dependencies. This led to
```plaintext
PS C:\Users\Bing\code\rust\blog> docker build -t blog .
[+] Building 471.3s (8/8) FINISHED
```
a *very* long wait.

The space issue wasn't *that bad*, but I really wanted to avoid waiting 10 minutes each time I tested a new change.

The next version was much better.
```Dockerfile
FROM rust:1.54 as build

# Create a new empty project
RUN USER=root cargo new --bin blog
WORKDIR /blog

# Copy project manifests
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# Build dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy source and build
COPY src src
RUN rm target/release/deps/blog*
RUN cargo build --release

CMD ["./target/release/blog"]

EXPOSE 80
```

In Docker, each command creates a "layer", which is cached. Docker only updates a layer if it's modified. If the `Cargo.toml` is not modified, then the dependencies couldn't have changed, so they don't need to be rebuilt. This reduces the compile time by almost a factor of 10.

I still have the space issue to deal with, which I fixed with a multi-stage build.

```Dockerfile
FROM rust:1.54 as build
# Everything else same as earlier

FROM debian:buster-slim

COPY --from=build /blog/target/release/blog ./app

COPY images /blog/images
COPY styles /blog/styles
COPY fonts /blog/fonts
COPY templates templates
COPY articles articles
COPY Rocket.toml Rocket.toml

CMD ["./app"]

EXPOSE 80
```
Now instead of using having all the dependencies and build tools in the image, I create a new image and only copy what is needed for the app to run. Here's a screenshot showing the difference.

<img src="/images/build-a-blog/docker-images.png" alt="screenshot of docker desktop showing change in docker image size">

After these changes, `docker build` runs much faster (on subsequent runs), and also takes much less space.

🎉

## Images <a id="images" class="anchor"></a>
Unfortunately, the syntax for inserting an image and creating a hyperlink in markdown is the same,
```markdown
[image alt text](/cool/image.png)
[link text](https://example.com)
```
so the `pulldown-cmark` parser can't decide if it should make a link or an image, so it defaults to a link.

There is a workaround, as you might have noticed. I can still insert images by manually typing html tags and linking them to the image. So this "markdown"
```html
<img src="/images/build-a-blog/docker-images.png" alt="screenshot of docker desktop showing change in docker image size">
```
results in this image

<img src="/images/build-a-blog/docker-images.png" alt="screenshot of docker desktop showing change in docker image size">

It's just slightly more annoying.

## Syntax highlighting (maybe)