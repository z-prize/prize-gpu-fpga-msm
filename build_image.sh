script=${BASH_SOURCE[0]}
if [ $script == $0 ]; then
  echo "ERROR: You must source this script"
  exit 2
fi
if [ "$BUCKET_ID" -eq "" ]; then
  echo "ERROR: BUCKET_ID must be set."
fi

full_script=$(readlink -f $script)
script_name=$(basename $full_script)


function usage {
  echo -e "USAGE: source $script_name [-d|-dcp DCP_FILE.tar] [-h|-help]"
}

function help {
  info_msg "$script_name"
  info_msg " "
  info_msg "build an FPGA image for POC"
  info_msg " "
  info_msg "build_agfi.sh script will:"
  info_msg "  (1) configure aws key and region (us-east-1)"
  info_msg "  (2) copy the tar file into an existing s3 bucket (in that region)"
  info_msg "  (3) create an FGPA image and return the corresponding AFI/AGFI identifier for loading it"
  echo " "
  usage
}
# Process command line args
args=( "$@" )
for (( i = 0; i < ${#args[@]}; i++ )); do
  arg=${args[$i]}
  case $arg in
    -d|-dcp)
      tar_file=${args[$i+1]}
	i=$i+1
    ;;
    -h|-help)
      help
      return 0
    ;;
    *)
      err_msg "Invalid option: $arg\n"
      usage
      return 1
  esac
done

info_msg " "
info_msg "copying tar file $tar_file to bucket s3://$BUCKET_ID/$tar_file"
aws s3 cp $tar_file s3://$BUCKET_ID/$tar_file

info_msg " "
info_msg "creating afi, logs will be saved in s3://$BUCKET_ID/logs.txt"

aws ec2 create-fpga-image \
  --name zprize_msm_submission \
  --description "Submission for ZPRIZE" \
  --input-storage-location Bucket=$BUCKET_ID,Key=$tar_file \
  --logs-storage-location Bucket=$BUCKET_ID,Key=logs.txt > image-id.json
