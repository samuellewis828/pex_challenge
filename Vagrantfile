Vagrant.configure("2") do |config|
  config.vm.box = "fedora/28-cloud-base"
  config.vm.box_version = "20180425"
  config.vm.synced_folder "./", "/vagrant", type: "rsync", rsync__auto: true, rsync__exclude: ['target/']

  config.vm.provider "virtualbox" do |vb|
    vb.customize ["modifyvm", :id, "--memory", 2048]
  end

  config.vm.provision "shell", path: "bootstrap_vagrant.sh"

end
