APP_PKG := ./sal_app/
APP := $(APP_PKG)/target/x86_64-unknown-linux-musl/release/sal_app

KERNEL_DIR:=/code/mainline
ARCH_DIR=arch/x86/boot
KERNEL := $(KERNEL_DIR)/$(ARCH_DIR)/bzImage

# This makes the ISO once the initramfs is done
# Takes the SYSLINUX Work, and the configuration stuff for SYSLINUX
sal.iso: clean syslinux/bios/core/isolinux.bin ramfs/initramfs.gz
	-rm -rf isodir
	mkdir -p isodir/isolinux
	cp resources/isolinux.cfg isodir/isolinux
	cp resources/boot.txt isodir/isolinux
	cp syslinux/bios/core/isolinux.bin isodir/isolinux
	cp syslinux/bios/com32/elflink/ldlinux/ldlinux.c32 isodir/isolinux
	cp ramfs/initramfs.gz isodir/isolinux
	cp $(KERNEL) isodir/isolinux

	mkisofs -V LFSISO -o sal.iso -b isolinux/isolinux.bin -c isolinux/boot.cat \
		-no-emul-boot -boot-load-size 4 -boot-info-table -iso-level 3 -f -R isodir
	isohybrid sal.iso

# COMPILE THE STATIC APPLICATION
.PHONY: app
app:
	cd $(APP_PKG) && \
		cargo build --target x86_64-unknown-linux-musl --release \
		--features "tech_emp setup_network"

$(APP):
	$(MAKE) app


.PHONY: ramfs
ramfs:
	$(MAKE) ramfs/initramfs.gz

# GENERATE 
ramfs/initramfs.gz: $(APP)
	-rm -rf ramfs
	mkdir ramfs
	cp $(APP) ramfs/init
	cd ramfs && find . | cpio -H newc -o | gzip -9 > initramfs.gz 

# GET THE BOOLOADER
.PHONY:
getsys:
	-rm -rf syslinux
	wget https://mirrors.edge.kernel.org/pub/linux/utils/boot/syslinux/syslinux-6.03.tar.xz
	tar xf syslinux-6.03.tar.xz
	mv syslinux-6.03 syslinux
	rm syslinux-6.03.tar.xz

syslinux/bios/core/isolinux.bin:
	$(MAKE) getsys

# CREATES THE BUSYBOX VERSION OF OUR PRODUCT
.PHONY: busy
busy: clean app busyfs sal.iso

.PHONY: clean
clean:
	-rm -rf ramfs initramfs.gz sal.iso isodir

.PHONY: cleanapp
cleanapp:
	cd $(APP_PKG) && cargo clean


# CLEANS SYSLINUX - dont use very often
.PHONY: cleansys
cleansys:
	-rm -rf syslinux syslinux-6.03.tar.xz resources/busybox

.PHONY: cleanall
cleanall: clean cleanapp cleansys


#### BUSYBOX SPECIFIC STUFF
# GET BUSYBOX
.PHONY: busybox
busybox:
	wget https://www.busybox.net/downloads/binaries/1.31.0-i686-uclibc/busybox
	chmod 777 busybox
	mv busybox resources/

resources/busybox:
	$(MAKE) busybox

# GENERATE AN INITRAMFS w/ BUSYBOX WHICH WILL EXEC APP
busyfs: $(APP) resources/busybox
	-rm -rf ramfs
	mkdir -p ramfs/{bin,sbin,etc,proc,usr,usr/bin,usr/sbin,dev,sys,mnt,mnt/rootfs}
	cp $(APP) ramfs/app
	cp resources/init ramfs/init
	cp resources/busybox ramfs/bin/
	cd ramfs && find . ! -name "initramfs.gz" | cpio -H newc -o | gzip -9 > initramfs.gz
