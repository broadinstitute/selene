use crate::util::error::Error;
use crate::util::sh_util;

#[cfg(linux_installation)]
const VEP_WRAPPER_PATH: &str = "/usr/share/selene/bash/vep_wrapper.sh";

#[cfg(not(linux_installation))]
const VEP_WRAPPER_PATH: &str = "assets/bash/vep_wrapper.sh";

pub(crate) struct VepSetupArgs {
    vep_cmd: String,
    fasta_file: String,
    cache_dir: String,
    plugins_dir: String,
    dbnsfp: String,
}

pub(crate) struct VepArgs {
    input_file: String,
    output_file: String,
    warnings_file: String,
    vep_setup_args: VepSetupArgs
}

impl VepSetupArgs {
    pub(crate) fn new(vep_cmd: String, fasta_file: String, cache_dir: String, plugins_dir: String,
                      dbnsfp: String)
                      -> VepSetupArgs {
        VepSetupArgs { vep_cmd, fasta_file, cache_dir, plugins_dir, dbnsfp }
    }
}

impl VepArgs {
    pub(crate) fn new(input_file: String, output_file: String, warnings_file: String,
                      vep_setup_args: VepSetupArgs)
                      -> VepArgs {
        VepArgs {
            input_file, output_file, warnings_file, vep_setup_args
        }
    }
}

pub(crate) fn run_vep(args: VepArgs) -> Result<(), Error> {
    let cpus = "1";
    sh_util::run("sh",
                 &[VEP_WRAPPER_PATH, args.vep_setup_args.vep_cmd.as_str(),
                     args.input_file.as_str(), cpus, args.vep_setup_args.fasta_file.as_str(),
                     args.vep_setup_args.cache_dir.as_str(),
                     args.vep_setup_args.plugins_dir.as_str(), args.vep_setup_args.dbnsfp.as_str(),
                     args.output_file.as_str(), args.warnings_file.as_str()],
    )
}

// def runVep(inputFile: File, outputFile: File, warningsFile: File): Int = {
// val vepCmd = runSettings.vepCmd
// val cpus = 1
// val fastaFile = runSettings.fastaFile
// val cacheDir = runSettings.cacheDir
// val pluginsDir = runSettings.pluginsDir
// val dbNsfp = runSettings.dbNSFPFile
// val commandLine =
// s"bash ${scriptRepo.Files.vepWrapper} $vepCmd $inputFile $cpus $fastaFile $cacheDir $pluginsDir $dbNsfp " +
// s"$outputFile $warningsFile"
// commandLine.!
// }