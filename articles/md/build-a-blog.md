Build a Blog
74389

Why do I have a blog? How did I make it?

### Table of Contents
 - [Senior Project](#senior-project)
 - [Actually Making it](#actually-making-it)
 - [Images?](#images)

## Senior Project <a id="senior-project" class="anchor"></a>
At my high school, in order to graduate, each senior must complete a Senior Project with the help of a mentor. For 15 days, students do "an experiential exploration of a topic of interest to the individual student".

For my project, I wanted to make a website. Prior to making this blog, I had no experience with web development, so this was a nice way to see what it's like.

I decided to use Rust, because I haven't used it for an actual project yet. The backend framework I used was [rocket.rs](https://rocket.rs/). I used [handlebars.js](https://handlebarsjs.com/) for templating. It's currently hosted on AWS Lightsail Containers.

## Actually Making it <a id="actually-making-it" class="anchor"></a>
The first few days were spent creating the basic routes: reading articles, the home page, etc. These were relatively simple to make; `rocket.rs` made it quite easy.

Uploading articles turned out to be not as trivial as the rest of the routes. I went through a few iterations of how I wanted to get new articles onto the web server. First, I used a form on the website itself to upload articles directly. The problem with this method was that I had to worry about security, since I only want myself to be able to upload articles.

The next idea was to not upload through the website, but rather SSH into the web server and copy over the articles. This method bypasses the security issue of the previous method, which is great. The only problem is that you don't have SSH access into Lightsail Containers, so this method can't work.

The method I finally settled on was to simply create a new container with the new article inside, and push that to Lightsail. This method had none of the security issues of the first method, and it could be entirely automated with GitHub Actions and Lightsail's CLI. The only annoying thing was writing a JSON string inline into a command, as shown by my commit history:
```plaintext
775ee90 Forgot quotes
2f10ff3 Another missing `}`
a567127 Forgot quotes
8917168 Need to actually use the variable
4552763 Missed a `}`
...
9d38048 Add a test markdown file
4cc17b3 Test gh actions
```

Adding support for Markdown was quite trivial with the `pulldown-cmark` crate. It only took a few lines (and most of them were optional!).
```rust
let mut options = Options::empty();
options.insert(Options::ENABLE_STRIKETHROUGH);
options.insert(Options::ENABLE_TABLES);
let parser = Parser::new_ext(rest, options);
let mut output = String::new();
html::push_html(&mut output, parser);
```

Adding fonts and css was also relatively easy. I just set up a `FileServer` to serve the files, and it Just Worked.
```rust
fn launch() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![index, get_article::article_page, get_article::articles],
        )
        .mount("/styles", FileServer::from(relative!("styles")))
        .mount("/fonts", FileServer::from(relative!("fonts")))
        .mount("/images", FileServer::from(relative!("images")))
        .attach(Template::fairing())
}
```

## Images <a id="images" class="anchor"></a>
Unfortunately, the syntax for inserting an image and creating a hyperlink in markdown is the same,
```markdown
[image alt text](/cool/image.png)
[link text](https://example.com)
```
so the `pulldown-cmark` parser can't decide if it should make a link or an image, so it defaults to a link.

There is a workaround, as you might have noticed. I can still insert images by manually typing html tags and linking them to the image. So this "markdown"
```html
<img src="/images/2bdab.png" alt="test">
```
results in this image

<img src="/images/2bdab.png" alt="test">

It's just slightly more annoying.