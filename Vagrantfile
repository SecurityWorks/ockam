Vagrant.configure("2") do |config|

  config.vm.define "builder-debian", primary: true do |config|
    config.vm.box = "ockam-network/builder-debian"

    boxes_url = "https://ockam.blob.core.windows.net/boxes"
    sha256 = "5f529cd919a9b3d83adc991b0460997892c004d5ad46131c2da3f7c6e66234ab"
    config.vm.box_url = "#{boxes_url}/builder/debian/#{sha256}.box"
    config.vm.box_download_checksum = sha256
    config.vm.box_download_checksum_type = "sha256"
    config.vm.box_check_update = false

    config.ssh.insert_key = false

    config.vm.provider :virtualbox do |vbox|
      vbox.name = "builder-debian"
      vbox.linked_clone = true
    end

    config.vm.synced_folder ".", "/vagrant", type: :rsync, rsync__exclude: ['.git/', 'tools/builder/']
  end

  ockam_use_macos_builder = ENV['OCKAM_USE_MACOS_BUILDER']
  if ockam_use_macos_builder
    ockam_private_boxes_shared_access_token = ENV['OCKAM_PRIVATE_BOXES_SHARED_ACCESS_TOKEN']

    if ockam_private_boxes_shared_access_token
      config.vm.define "builder-macos", primary: true do |config|
        config.vm.box = "ockam-network/builder-macos"

        boxes_url = "https://ockam.blob.core.windows.net/boxes-private"
        sha256 = "355dff4ee9783adae79a1d4778693e3ffd7f765f5caebb4cd74501d59fcf7a77"
        config.vm.box_url = "#{boxes_url}/builder/macos/#{sha256}.box?#{ockam_private_boxes_shared_access_token}"
        config.vm.box_download_checksum = sha256
        config.vm.box_download_checksum_type = "sha256"
        config.vm.box_check_update = false

        config.vm.provider :virtualbox do |vbox|
          vbox.name = "builder-macos"
          vbox.linked_clone = true
        end

        config.vm.synced_folder ".", "/vagrant", type: :rsync, rsync__exclude: ['.git/', 'tools/builder/']
      end
    end

  end
end
