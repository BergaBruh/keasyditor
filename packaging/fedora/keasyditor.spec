Name:           keasyditor
Version:        0.1.0
Release:        1%{?dist}
Summary:        Visual editor for Klassy and Kvantum KDE themes
License:        MIT
BuildArch:      x86_64
URL:            https://github.com/BergaBruh/keasyditor
AutoReqProv:    no

Requires:       libxkbcommon
Requires:       wayland-libs-client
Requires:       fontconfig-libs
Requires:       mesa-libGL
Requires:       mesa-libEGL

%description
KEasyDitor provides a graphical interface for editing KDE Plasma themes.

Supports the Klassy window decoration engine (klassyrc) and the Kvantum
Qt widget theme engine (.kvconfig + SVG). Features live preview, undo/redo,
and colour palette extraction via matugen.

%prep
# Binary is pre-built by cargo in the Docker builder stage.

%build
# No-op.

%install
install -Dm755 %{_builddir}/keasyditor \
               %{buildroot}%{_bindir}/keasyditor
install -Dm644 %{_builddir}/keasyditor.desktop \
               %{buildroot}%{_datadir}/applications/keasyditor.desktop
install -Dm644 %{_builddir}/keasyditor.svg \
               %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/keasyditor.svg

%files
%{_bindir}/keasyditor
%{_datadir}/applications/keasyditor.desktop
%{_datadir}/icons/hicolor/scalable/apps/keasyditor.svg

%post
gtk-update-icon-cache -f -t %{_datadir}/icons/hicolor &>/dev/null || :

%postun
gtk-update-icon-cache -f -t %{_datadir}/icons/hicolor &>/dev/null || :

%changelog
* Thu Apr 10 2026 KEasyDitor contributors <build@keasyditor.local> - 0.1.0-1
- Initial package
