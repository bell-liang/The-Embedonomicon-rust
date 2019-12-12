SECTIONS
{
  .log 0 (INFO) : {
    *(.log);
    *(.log.error);
    __log__warning_start__ = .;
    *(.log.warning);
  }
}