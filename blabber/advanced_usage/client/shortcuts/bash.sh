blabber_flags="blabber_username=USERNAME blabber_password=PASSWORD blabber_url=URL"
blabber_repo="REPO_PATH"
blabber_device="pc OR termux"
blabu() {
    $blabber_flags "$blabber_repo/client.py" "$1"
}
blabn() {
    $blabber_flags "$blabber_repo/advanced_usage/client/notifiers/notify-$blabber_device" "$1"
}
