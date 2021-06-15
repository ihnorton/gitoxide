mod registration {
    use std::path::Path;
    fn filecount_in(path: impl AsRef<Path>) -> usize {
        std::fs::read_dir(path).expect("valid dir").count()
    }

    mod signals {
        use crate::tempfile::registration::filecount_in;

        #[test]
        fn various_termination_signals_remove_tempfiles_unconditionally() -> crate::Result {
            let dir = tempfile::tempdir()?;
            for sig in signal_hook::consts::TERM_SIGNALS {
                let _tempfile = git_tempfile::new(dir.path())?;
                assert_eq!(
                    filecount_in(dir.path()),
                    1,
                    "only one tempfile exists no matter the iteration"
                );
                signal_hook::low_level::raise(*sig)?;
                assert_eq!(
                    filecount_in(dir.path()),
                    0,
                    "the signal triggers removal but won't terminate the process (anymore)"
                );
            }
            Ok(())
        }
    }
    mod at_path {
        #[test]
        fn it_names_files_correctly_and_removes_them_when_out_of_scope() -> crate::Result {
            let dir = tempfile::tempdir()?;
            let filename = dir.path().join("something-specific.ext");
            let tempfile = git_tempfile::at_path(&filename)?;
            assert!(filename.is_file(), "specified file should exist precisely");
            drop(tempfile);
            assert!(!filename.is_file(), "after drop named files are deleted as well");
            Ok(())
        }
    }

    mod new {
        use crate::tempfile::registration::filecount_in;

        #[test]
        fn it_can_be_kept() -> crate::Result {
            let dir = tempfile::tempdir()?;
            drop(git_tempfile::new(dir.path())?.take().expect("not taken yet").keep()?);
            assert_eq!(filecount_in(&dir), 1, "a temp file and persisted");
            Ok(())
        }

        #[test]
        fn it_is_removed_if_it_goes_out_of_scope() -> crate::Result {
            let dir = tempfile::tempdir()?;
            {
                let _keep = git_tempfile::new(dir.path());
                assert_eq!(filecount_in(&dir), 1, "a temp file was created");
            }
            assert_eq!(filecount_in(&dir), 0, "tempfile was automatically removed");
            Ok(())
        }
    }
}

mod force_setup {
    #[test]
    fn can_be_called_multiple_times() {
        // we could probably be smart and figure out that this does the right thing, but… it's good enough it won't fail ;).
        git_tempfile::force_setup(git_tempfile::SignalHandlerMode::HandleTermination);
        git_tempfile::force_setup(git_tempfile::SignalHandlerMode::HandleTerminationAndRestoreDefaultBehaviour);
    }
}