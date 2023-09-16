<?php

function translate($language, $text)
{
    $languages = array(
        'ja', 'ko', 'ru', 'zh', 'en', 'it', 'es', 'fr', 'de', 'tr', 'az', 'el'
    );

    $translations = include($language . '.php');

    if (isset($translations[$text])) {
        return $translations[$text];
    } else {
        return 'Translation not found';
    }
}

$translatedText = translate('az', 'Info: Violet search is not saving your search history.');
echo $translatedText;
