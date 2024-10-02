pkg install neovim rust git wget -y
git clone https://github.com/NvChad/starter ~/.config/nvim && nvim
git clone https://github.com/adixek/Fake.git && cd Fake && cp font.ttf ~/.termux/font.ttf && cp rs ~/../usr/bin && chmod +x ~/../usr/bin/rs && cd && rm -rf Fake 

