# Language Documentation

## Table of Contents
* [Introduction](#introduction)
* [Adding a Language](#adding-a-language)
* [Adding a Country Code for Search Engines and Wikipedia](#adding-a-search-engine)

## Introduction
This document will explain how to add a language to Violet Search. This process requires some knowledge of PHP(Laravel) and Rust.

## Adding a Language
1. Create a new directory in `resources/lang` with the language code as the name. For example, if you want to add Serbian, you would create a PHP file called `rs.php`.
2. Copy the contents of `resources/lang/en.php` into the new file.
3. Translate the strings in the new file.
4. Open 'resources/lang/translate.php' and add the language code to the array. For example, if you added Serbian, you would add `rs`.

Example using (translate.php):
```php
<?php
function translate($language, $text)
{
    // our country code is 'rs'
    $languages = array(
        'ja', 'ko', 'ru', 'zh', 'en', 'it', 'es', 'fr', 'de', 'tr', 'az', 'el',/* adding rs file */ 'rs'
    );

    $translations = include($language . '.php');

    if (isset($translations[$text])) {
        return $translations[$text];
    } else {
        return 'Translation not found';
    }
}

/*
    Then you can use the function like this. The sentence to be translated must be in English.
*/
$translatedText = translate('rs', 'Info: Violet search is not saving your search history.');
echo $translatedText;

```

## Adding a Search Engine and Wikipedia
...
