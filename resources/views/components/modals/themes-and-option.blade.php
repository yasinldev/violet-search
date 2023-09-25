<div class="d-flex justify-center">
    <script src="https://cdnjs.cloudflare.com/ajax/libs/ace/1.27.0/ace.js" integrity="sha512-7UWnxa401enUNZ8plgMQJz1EPcNhkPJxI3OsnzlDXsw6Y58hEZMNMSVnkkWAKyuSxtaAE0pss4KJAl5+D403gw==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
    <div class="modal" id="themes_modal">
        <div class="violet-row">
            <div class="column-na-12">
                <b class="modal-title suspend-text font-assistant" style="margin-top: 10px">
                    <a class="modal-close text-end suspend-text" href="#" id="modal-close">
                        <i class="bi bi-x-lg"></i>
                    </a>
                    <i class="bi bi-palette"></i> <b>Themes and Options</b>
                </b>
            </div>
        </div>
        <div class="modal-content">
            <div class="violet-row">
                <div class="column-na-12">
                    <div class="code-editor font-assistant" style="font-family: monospace, monospace" id="editor">
                        <div id="editor_text"></div>
                    </div>
                    <div class="d-flex responsive-margin-bottom">
                        <div class="mt-10 responsive-margin-right font-assistant other-buttons text-center" id="save_editor">
                            save
                        </div>
                        <div class="mt-10 font-assistant justify-end other-buttons text-center" id="reset_editor">
                            reset
                        </div>
                    </div>
                    <input type="url" id="wallpaper_input" class="input" placeholder="Enter a Wallpaper link" />
                    <div class="d-flex">
                        <div id="save_wallpaper" class="responsive-margin-right font-assistant other-buttons text-center">
                            save
                        </div>
                        <div id="reset_wallpaper" class="responsive-margin-right font-assistant other-buttons text-center">
                            reset
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
