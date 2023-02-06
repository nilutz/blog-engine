

# A Blog engine

This is a simple static site generator. The work is based on [Zola](https://www.getzola.org/) and [this](https://patshaughnessy.net/2019/9/4/using-rust-to-build-a-blog-site) blog post by Pat Shaughnessy. I wanted to a blog for a long time and I wanted to learn rust for a long time. So I started learning rust by building this static site generator, which in turn powers this blog. The engine takes markdown files in `/notes` and  [tera](https://tera.netlify.app) templates (localted in `/templates`) then it turns them into plain old html. Simple as that.


# Deployment
You can either drag-and-drop via the cloudflare UI or use wrangler. The Makefile has a convenient cmd setup:

    make publish_with_wrangler



# Future

A more dev friendly experience:

* watches if markdown changes: https://dev.to/shinshin86/let-rust-detect-changes-in-the-markdown-file-and-generate-html-2a8e and recompile the html instantly

* mathJax rendering
* more syntax highlighting

* bug fix >< symbols
