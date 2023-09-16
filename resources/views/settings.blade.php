<!doctype html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport"
          content="width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">
    <title>Violet Search - Settings</title>

    <!-- Assets -->
    <link rel="icon" href="{{ asset('image/violet.png') }}">
    <link rel="stylesheet" href="{{ asset('css/app.css') }}">

    <!-- Bootstrap Icons -->
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.10.5/font/bootstrap-icons.css">
</head>
<body>
    <div class="violet-container is-fluid">
        <div class="violet-row">
            <div class="column-na-3">
                <div class="settings">
                    <div class="title font-assistant">
                        <img src="{{ asset('image/violet.png') }}" alt="violet-search" /> &nbsp; &nbsp; Violet Search
                    </div>
                    <br><br>
                    <b class="on-padding font-assistant suspend-text">
                        <i class="bi bi-gear"></i> &nbsp; Settings
                    </b>
                    <br><br>
                    <div class="sidebar-item suspend-text font-assistant">
                        <a href="{{ route('settings') }}?param=themes_and_options">
                            <i class="bi bi-palette"></i> <b>Themes and Options</b>
                        </a>
                    </div>
                    <div class="sidebar-item suspend-text font-assistant">
                        <a href="{{ route('settings') }}?param=language_and_results">
                            <i class="bi bi-geo-alt"></i> <b>Language and Results</b>
                        </a>
                    </div>
                    <div class="sidebar-item suspend-text font-assistant">
                        <a href="{{ route('settings') }}?param=tor_connection">
                            <i class="bi bi-bezier"></i> <b>Tor Connection</b>
                        </a>
                    </div>
                    <div class="sidebar-item suspend-text font-assistant">
                        <a href="{{ route('settings') }}?param=developers">
                            <i class="bi bi-code"></i> <b>Developers</b>
                        </a>
                    </div>
                    <br><br>
                    <b class="on-padding font-assistant suspend-text">
                        <i class="bi bi-info-circle"></i> &nbsp; Legal
                    </b>
                    <br><br>
                    <div class="sidebar-item suspend-text font-assistant">
                        <a href="{{ route('settings') }}?param=privacy_policy">
                            <i class="bi bi-file-earmark-lock"></i> <b>Privacy Policy</b>
                        </a>
                    </div>
                    <div class="sidebar-item suspend-text font-assistant">
                        <a href="{{ route('settings') }}?param=terms_of_service">
                            <i class="bi bi-file-earmark-text"></i> <b>Terms of Service</b>
                        </a>
                    </div>
                    <br><br>
                    <b class="on-padding font-assistant suspend-text">
                        <i class="bi bi-people"></i> &nbsp; Community
                    </b>
                    <br><br>
                    <div class="sidebar-item suspend-text font-assistant">
                        <a href="https://github.com/violet-eco/violet-search/blob/master/CONTRIBUTE.md">
                            <i class="bi bi-activity"></i> <b>Contribute</b>
                        </a>
                    </div>
                    <div class="sidebar-item suspend-text font-assistant">
                        <a href="https://github.com/violet-eco/violet-search/issues">
                            <i class="bi bi-bug"></i> <b>Create a issue</b>
                        </a>
                    </div>
                    <div class="sidebar-item suspend-text font-assistant">
                        <a href="https://github.com/violet-eco/violet-search">
                            <i class="bi bi-github"></i> <b>GitHub</b>
                        </a>
                    </div>
                </div>
            </div>
            <div class="column-na-9 mt-10">
                <?php
                    if (!isset($_GET['param'])) { ?>
                       <x-settings.welcome />
                <?php } ?>
                <?php
                    if (isset($_GET['param'])) {
                        if ($_GET['param'] == 'themes_and_options') { ?>
                            <x-settings.themes_and_options />
                <?php   }
                    }
                ?>
            </div>
        </div>
    </div>
</body>
</html>
