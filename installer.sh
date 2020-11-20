#!/usr/bin/env bash

echo
echo
echo "What do you want to do ?"
echo "1) Install" ; echo "2) Update" ; echo "3) Exit"
read -p 'Type the number : ' choice

if [ $choice = '1' ];then
		if command -v vlugger &> /dev/null
		then
				echo "Vlugger is already installed"
				exit
		fi
		cargo install vlugger
elif [ $choice = '2' ];then

		if ! command -v vlugger &> /dev/null
		then
				echo "vlugger is not installed. Please install it and then update"
				exit
		fi

		VERSION_REGEX="\"(.*?)\"" # Regex to get version on crates.io
		TEMP_VERSION=$(cargo search vlugger | egrep -o ${VERSION_REGEX}) # Get vlugger version on crates.io and apply regex on it
		CURRENT_VERSION=$(vlugger -v) # Get the version of vlugger installed on the computer
		CRATES_IO_VERSION="${TEMP_VERSION%\"}" # Removing quotes at the beggining of the version name
		CRATES_IO_VERSION="${CRATES_IO_VERSION#\"}" # Removing quotes at the end of the version name

		if [[ ${CRATES_IO_VERSION} == ${CURRENT_VERSION} ]];then
				echo "Already up to date"
		else
				echo "Updating..."
				WD=$(pwd)
				cd /bin/
				echo "Downloading version ${CRATES_IO_VERSION} from crates.io..."

				cargo install vlugger

				if [[ $? == 0 ]];then
						echo "Successfully downloaded latest version"
				else
						echo "Error in process. Please retry later"
				fi
				cd ${WD}
		fi
fi
