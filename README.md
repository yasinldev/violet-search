<div align="center">
<p><img width="150" src="resources/image/violet.png"></p>

# Violet Search Engine

<p align="center">
  <em>Privacy respecting, customizable search engine</em><br>
  <b>Your Gateway to Secure Discovery</b><br><br>
  Violet is a search engine that respects your privacy and allows you to customize your search experience. Violet is not a replacement for Google, but rather a tool to help you find what you are looking for without being tracked.
</p>
</div>

> Warning: Violet is still in development and should not be used in production. We are working hard to make Violet the best search engine out there, so if you have any feedback or suggestions, please let us know! can you create a issue on GitHub.

## Why Violet?

Because we believe that privacy is a fundamental human right that should be protected at all costs. We also believe that the internet should be a place where you can freely express yourself without being tracked or censored. Violet is our way of helping you achieve that.

## Features

- **Privacy Respecting**: Violet does not track you or your searches. We do not store your IP address, your search history, or any other information that could be used to identify you.
- **Strong Language Support**: Violet supports multiple languages, including English, French, German, Spanish, Italian, Portuguese, Greek, Chinese, Japanese, Korean, Turkish, and Azerbaijani.
- **Fast and Responsive**: Violet is fast and responsive. It is built using Rust and PHP, which are both fast and efficient programming languages. This means that Violet can handle a large number of requests without slowing down or crashing.
- **Meta Search or Own Results**: Violet allows you to choose between meta search results or your own results. Meta search results are provided by Google, Brave, DuckDuckGo, and other search engines. Your own results are provided by Violet's own crawler
- **Customizable**: Violet allows you to customize your search experience. You can choose from a variety of search engines, including Google, Bing, DuckDuckGo results. You can also choose to block certain websites from appearing in your search results.
- **Strong and Reliable Infrastructure**: Violet is developed using robust programming languages like Rust and PHP. Rust, in particular, offers advantages in terms of memory safety and performance, which can lead to a more stable and faster experience.
- **No Ads**: Violet does not show any ads. We do not track you, so we cannot show you ads. We do not make any money from Violet. We do not sell your data to third parties. We do not have any investors. We do not have any sponsors. We do not have any ads. Violet is completely free to use.
- **Clean Code**: Violet is written in PHP and uses the Laravel framework. The code is clean and easy to read. We have also included a lot of comments to help you understand what is going on...

## Process

Violet is having two main components: the search engine and the microservice. The search engine is written in PHP and uses the Laravel framework. The microservice is written in Rust. Here is a diagram of how the two components interact with each other (simple version): <br><br>
<img src="resources/image/violet_d.png"><br><br>
The microservice is responsible for crawling the web and indexing the pages. The search engine is responsible for serving the search results to the user.

## FAQ (Frequently Asked Questions)

### What is Violet?

Violet is a search engine that respects your privacy and allows you to customize your search experience. Violet is responsive, fast, and easy to use. It is also open source and free to use.
### How does Violet work?

Violet is using a scraper to crawl the web and index the pages. The search engine is responsible for serving the search results to the user. The scraper is written in Rust and the search engine is written in PHP using the Laravel framework.

### Should I use Violet Search?

If you are looking for a search engine that respects your privacy and allows you to customize your search experience, then yes, you should use Violet Search.

### Why should I use Violet Search instead of Searx?

There are several reasons why you might consider using Violet Search instead of Searx:
* <b>Strong and Reliable Infrastructure:</b> Violet Search is developed using robust programming languages like Rust and PHP. Rust, in particular, offers advantages in terms of memory safety and performance, which can lead to a more stable and faster experience.
* <b>Greater Customization Options:</b> Violet Search allows users to customize their search experience to a greater extent. This means you can configure the search engine according to your preferences. These customization options enable users to personalize search results, filters, and other features.
* <b>Privacy and Security Focus:</b> Violet Search prioritizes user privacy and data security. <b>Like Searx,</b> it commits to not tracking or storing user data.
* <b>Language Support:</b> Violet Search supports multiple languages, including English, French, German, Spanish, Italian, Portuguese, Greek, Chinese, Japanese, Korean, Turkish, and Azerbaijani.
* <b>Own Results:</b> Violet Search allows users to choose between meta search results or their own results. Meta search results are provided by Google, Brave, DuckDuckGo, and other search engines. Your own results are provided by Violet's own crawler.


## Supported Languages and Search Engines

### Languages

- English
- French
- Germans
- Spanish
- Italian
- Portuguese
- Greek
- Chinese
- Japanese
- Korean
- Turkish
- Azerbaijani

If you would like to add support for another language, please open an issue on GitHub.
<br><br>
Or you can add it yourself by editing the `resources/lang` directory.
<br>
You can find more information about how to do this in the [Violet Language Documantation](LANGUAGE.md).
### Search Engines

- Google
- DuckDuckGo
- Artado
- Qwant
- Yahoo
- Swisscows
- Startpage (Only results)
- Ecosia
- Ask
- Brave
- Violet Results (In Development)

## Setup and Setting Up Microservice

### Requirements

- PHP 7.4 or higher
- MySQL 5.7 or higher
- Composer
- Rust

### Installation search engine

1. Clone the repository

```bash
git clone https://github.com/violet-eco/violet-search
```

2. Install dependencies

```bash
composer install
```

3. Enter violet-search dir

```bash
cd violet-search
```
4. Copy the `.env.example` file to `.env` and update the database credentials

```bash
cp .env.example .env
```

5. Generate a new application key

```bash
php artisan key:generate
```

6. Start the development server

```bash
php artisan serve
```

### Setting Up Microservice

2. Enter the microservice directory

```bash
cd microservice
```

3. Build the microservice

```bash
cargo build --release
```

4. Run the microservice

```bash
cargo run --release
```

## Contributing

Contributions are always welcome! If you have any suggestions or would like to contribute to the project, please open an issue on GitHub. You can also fork the repository and submit a pull request.

Please read the [CONTRIBUTING.md](CONTRIBUTE.md) file for more information.

## License

Violet is licensed under the GPL-3.0 License. See [LICENSE](LICENSE) for more information.
