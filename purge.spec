Name:           purge
Version:        1.2.0
Release:        1%{?dist}
Summary:        A fast, native GUI for removing bloatware from Android devices.

License:        MIT
URL:            https://github.com/Md-Siam-Mia-Man/UAD-Universal-Android-Debloater
Source0:        %{url}/archive/refs/tags/v%{version}.tar.gz

BuildRequires:  rust
BuildRequires:  cargo
BuildRequires:  gtk3-devel
BuildRequires:  libappindicator-devel

%description
P.U.R.G.E. (Package Uninstaller and Resource Garbage Eliminator) helps you
reclaim your device's privacy and battery life by making it simple to
remove unnecessary system apps.

%prep
%setup -q -n %{name}-%{version}

%build
cargo build --release --locked

%install
install -Dm755 %{_builddir}/%{name}-%{version}/target/release/%{name} %{buildroot}%{_bindir}/%{name}

%files
%{_bindir}/%{name}
%license LICENSE.md

%changelog
* Sun Jan 01 2024 Md. Siam Mia <your-email@example.com> - 1.2.0-1
- Initial RPM packaging